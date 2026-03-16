use proc_macro::TokenStream;
use quote::quote;
use syn::Attribute;
use syn::Data;
use syn::DeriveInput;
use syn::Fields;
use syn::Meta;
use syn::parse_macro_input;

#[proc_macro_derive(Deserialize, attributes(xpath, serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Get the root xpath from struct attributes
    let root_xpath = get_xpath_attr(&input.attrs).unwrap_or_default();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let mut field_descriptors = Vec::new();
    let mut field_deserializations = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();

        let (xpath, is_text) = get_field_xpath_attr(&field.attrs);
        let xpath = xpath.unwrap_or_default();

        let has_serde_default = has_serde_default_attr(&field.attrs);
        let field_type = &field.ty;

        // Determine field kind based on type and attributes
        let (kind, is_optional, is_vec) = determine_field_kind(
            field_type,
            is_text,
            has_serde_default,
            &xpath,
        );

        let kind_tokens = match kind.as_str() {
            "Attribute" => {
                quote! { serde_xml::__private::FieldKind::Attribute }
            }
            "Text" => quote! { serde_xml::__private::FieldKind::Text },
            "Sequence" => quote! { serde_xml::__private::FieldKind::Sequence },
            "Optional" => quote! { serde_xml::__private::FieldKind::Optional },
            "OptionalSequence" => {
                quote! { serde_xml::__private::FieldKind::OptionalSequence }
            }
            _ => quote! { serde_xml::__private::FieldKind::Element },
        };

        field_descriptors.push(quote! {
            serde_xml::__private::FieldDescriptor {
                name: #field_name_str,
                xpath: #xpath,
                kind: #kind_tokens,
            }
        });

        // Generate field deserialization code
        let deser_code = if is_vec {
            // Vec field - use sequence deserialization
            let inner_type = extract_inner_type(field_type, "Vec");
            if let Some(inner) = inner_type {
                if is_simple_type(&inner) {
                    quote! {
                        let #field_name: #field_type = deser.deserialize_field(#field_name_str)?;
                    }
                } else {
                    // Nested struct in Vec
                    generate_vec_nested_deser(
                        field_name,
                        &field_name_str,
                        &xpath,
                        &inner,
                    )
                }
            } else {
                quote! {
                    let #field_name: #field_type = deser.deserialize_field(#field_name_str)?;
                }
            }
        } else if is_optional {
            // Option field
            let inner_type = extract_inner_type(field_type, "Option");
            if let Some(inner) = inner_type {
                if is_simple_type(&inner) {
                    generate_optional_simple_deser(
                        field_name,
                        &field_name_str,
                        &xpath,
                        &inner,
                        is_text,
                    )
                } else {
                    // Nested struct in Option
                    generate_optional_nested_deser(
                        field_name,
                        &field_name_str,
                        &xpath,
                        &inner,
                    )
                }
            } else {
                quote! {
                    let #field_name: #field_type = deser.deserialize_field(#field_name_str)?;
                }
            }
        } else if is_text || xpath.starts_with("/@") {
            // Simple text or attribute
            quote! {
                let #field_name: #field_type = deser.deserialize_field(#field_name_str)?;
            }
        } else if is_simple_type(field_type) {
            quote! {
                let #field_name: #field_type = deser.deserialize_field(#field_name_str)?;
            }
        } else {
            // Nested struct
            generate_nested_struct_deser(
                field_name,
                &field_name_str,
                &xpath,
                field_type,
            )
        };

        field_deserializations.push(deser_code);
    }

    let field_names: Vec<_> =
        fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

    let expanded = quote! {
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                // This path is used when deserializing from a non-XPath deserializer
                // For now, return an error suggesting to use serde_xml::from_str
                Err(serde::de::Error::custom(
                    "use serde_xml::from_str to deserialize this type"
                ))
            }
        }

        impl serde_xml::FromXml for #name {
            fn from_xml(xml: &str) -> std::result::Result<Self, serde_xml::Error> {
                Self::__deserialize_from_xml(xml)
            }
        }

        impl #name {
            const __XPATH_DESCRIPTOR: serde_xml::__private::StructDescriptor =
                serde_xml::__private::StructDescriptor {
                    name: stringify!(#name),
                    root_xpath: #root_xpath,
                    fields: &[
                        #(#field_descriptors),*
                    ],
                };

            #[doc(hidden)]
            pub fn __deserialize_from_xml(
                xml: &str,
            ) -> std::result::Result<Self, serde_xml::Error> {
                serde_xml::from_str_with_descriptor(
                    xml,
                    &Self::__XPATH_DESCRIPTOR,
                    |deser| {
                        #(#field_deserializations)*

                        Ok(#name {
                            #(#field_names),*
                        })
                    },
                )
            }

            #[doc(hidden)]
            pub fn __deserialize_from_node<'a, 'input>(
                node: roxmltree::Node<'a, 'input>,
                parent_descriptor: &'static serde_xml::__private::StructDescriptor,
            ) -> std::result::Result<Self, serde_xml::Error> {
                serde_xml::__private::deserialize_struct_from_node(
                    node,
                    &Self::__XPATH_DESCRIPTOR,
                    |deser| {
                        #(#field_deserializations)*

                        Ok(#name {
                            #(#field_names),*
                        })
                    },
                )
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_xpath_attr(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("xpath")
            && let Meta::List(meta_list) = &attr.meta
        {
            let tokens = meta_list.tokens.to_string();
            // Parse the string literal
            if let Some(s) =
                tokens.strip_prefix('"').and_then(|s| s.strip_suffix('"'))
            {
                return Some(s.to_string());
            }
        }
    }
    None
}

fn get_field_xpath_attr(attrs: &[Attribute]) -> (Option<String>, bool) {
    for attr in attrs {
        if attr.path().is_ident("xpath")
            && let Meta::List(meta_list) = &attr.meta
        {
            let tokens = meta_list.tokens.to_string();
            // Check if it contains serde_xml::Text
            let is_text =
                tokens.contains("serde_xml::Text") || tokens.contains("Text");

            // Parse the xpath string (first argument)
            let parts: Vec<&str> = tokens.split(',').collect();
            if let Some(first) = parts.first() {
                let first = first.trim();
                if let Some(s) =
                    first.strip_prefix('"').and_then(|s| s.strip_suffix('"'))
                {
                    return (Some(s.to_string()), is_text);
                }
            }
        }
    }
    (None, false)
}

fn has_serde_default_attr(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("serde")
            && let Meta::List(meta_list) = &attr.meta
        {
            let tokens = meta_list.tokens.to_string();
            if tokens.contains("default") {
                return true;
            }
        }
    }
    false
}

fn determine_field_kind(
    ty: &syn::Type,
    is_text: bool,
    has_default: bool,
    xpath: &str,
) -> (String, bool, bool) {
    let type_str = quote!(#ty).to_string();

    let is_vec = type_str.starts_with("Vec <") || type_str.starts_with("Vec<");
    let is_option =
        type_str.starts_with("Option <") || type_str.starts_with("Option<");

    // Check if this is an attribute xpath (ends with /@attr or is just /@attr)
    let is_attribute = xpath.contains("/@");

    if is_text {
        return ("Text".to_string(), is_option, is_vec);
    }

    if is_attribute {
        return ("Attribute".to_string(), is_option, is_vec);
    }

    if is_vec {
        if is_option || has_default {
            return ("OptionalSequence".to_string(), false, true);
        }
        return ("Sequence".to_string(), false, true);
    }

    if is_option || has_default {
        return ("Optional".to_string(), true, false);
    }

    ("Element".to_string(), false, false)
}

fn extract_inner_type(ty: &syn::Type, wrapper: &str) -> Option<syn::Type> {
    if let syn::Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
        && segment.ident == wrapper
        && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
    {
        return Some(inner.clone());
    }
    None
}

fn is_simple_type(ty: &syn::Type) -> bool {
    let type_str = quote!(#ty).to_string();
    matches!(
        type_str.as_str(),
        "String"
            | "str"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "f32"
            | "f64"
            | "bool"
            | "char"
            | "& str"
            | "& 'static str"
    )
}

fn generate_nested_struct_deser(
    field_name: &syn::Ident,
    field_name_str: &str,
    xpath: &str,
    field_type: &syn::Type,
) -> proc_macro2::TokenStream {
    quote! {
        let #field_name: #field_type = {
            let xpath = serde_xml::xpath::XPath::parse(#xpath)
                .map_err(|e| serde_xml::Error::XPath(e))?;
            let result = xpath.evaluate_single(deser.node())
                .ok_or_else(|| serde_xml::Error::MissingField(#field_name_str.to_string()))?;
            let node = result.as_node()
                .ok_or_else(|| serde_xml::Error::XPath(format!("expected element for field '{}'", #field_name_str)))?;
            #field_type::__deserialize_from_node(node, &Self::__XPATH_DESCRIPTOR)?
        };
    }
}

fn generate_optional_nested_deser(
    field_name: &syn::Ident,
    _field_name_str: &str,
    xpath: &str,
    inner_type: &syn::Type,
) -> proc_macro2::TokenStream {
    quote! {
        let #field_name: Option<#inner_type> = {
            let xpath = serde_xml::xpath::XPath::parse(#xpath)
                .map_err(|e| serde_xml::Error::XPath(e))?;
            match xpath.evaluate_single(deser.node()) {
                Some(result) => {
                    match result.as_node() {
                        Some(node) => Some(#inner_type::__deserialize_from_node(node, &Self::__XPATH_DESCRIPTOR)?),
                        None => None,
                    }
                }
                None => None,
            }
        };
    }
}

fn generate_optional_simple_deser(
    field_name: &syn::Ident,
    _field_name_str: &str,
    xpath: &str,
    inner_type: &syn::Type,
    is_text: bool,
) -> proc_macro2::TokenStream {
    if is_text {
        quote! {
            let #field_name: Option<#inner_type> = {
                let xpath = serde_xml::xpath::XPath::parse(#xpath)
                    .map_err(|e| serde_xml::Error::XPath(e))?;
                match xpath.evaluate_single(deser.node()) {
                    Some(result) => {
                        result.text().map(|s| s.to_string())
                    }
                    None => None,
                }
            };
        }
    } else {
        quote! {
            let #field_name: Option<#inner_type> = {
                let xpath = serde_xml::xpath::XPath::parse(#xpath)
                    .map_err(|e| serde_xml::Error::XPath(e))?;
                match xpath.evaluate_single(deser.node()) {
                    Some(result) => {
                        match result.as_str() {
                            Some(s) => Some(s.to_string()),
                            None => result.text().map(|s| s.to_string()),
                        }
                    }
                    None => None,
                }
            };
        }
    }
}

fn generate_vec_nested_deser(
    field_name: &syn::Ident,
    _field_name_str: &str,
    xpath: &str,
    inner_type: &syn::Type,
) -> proc_macro2::TokenStream {
    quote! {
        let #field_name: Vec<#inner_type> = {
            let xpath = serde_xml::xpath::XPath::parse(#xpath)
                .map_err(|e| serde_xml::Error::XPath(e))?;
            let results = xpath.evaluate_all(deser.node());
            let mut items = Vec::new();
            for result in results {
                if let Some(node) = result.as_node() {
                    items.push(#inner_type::__deserialize_from_node(node, &Self::__XPATH_DESCRIPTOR)?);
                }
            }
            items
        };
    }
}
