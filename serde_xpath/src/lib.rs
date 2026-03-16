pub mod de;
pub mod error;
pub mod xpath;

pub use de::__private;
pub use error::Error;
pub use serde_xpath_derive::Deserialize;

/// Marker type for extracting text content from an element
pub struct Text;

/// Trait for types that can be deserialized from XML using XPath
pub trait FromXml: Sized {
    fn from_xml(xml: &str) -> Result<Self, Error>;
}

/// Deserialize a type from an XML string
pub fn from_str<T: FromXml>(s: &str) -> Result<T, Error> {
    T::from_xml(s)
}

/// Deserialize a type from an XML string using the XPath-based deserializer
/// This is called by the generated Deserialize implementations
pub fn from_str_with_descriptor<T, F>(
    s: &str,
    descriptor: &__private::StructDescriptor,
    field_deserializer: F,
) -> Result<T, Error>
where
    F: FnOnce(__private::StructFieldDeserializer<'_, '_>) -> Result<T, Error>,
{
    __private::deserialize_xpath_struct(s, descriptor, field_deserializer)
}
