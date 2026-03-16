use roxmltree::Node;
use serde::de::Visitor;
use serde::de::{self};

use crate::error::Error;
use crate::xpath::XPath;
use crate::xpath::XPathResult;

pub struct XPathDeserializer<'a, 'input> {
    node: Node<'a, 'input>,
}

impl<'a, 'input> XPathDeserializer<'a, 'input> {
    pub fn new(node: Node<'a, 'input>) -> Self {
        XPathDeserializer { node }
    }

    pub fn from_node(node: Node<'a, 'input>) -> Self {
        Self::new(node)
    }
}

impl<'de, 'a, 'input> de::Deserializer<'de> for XPathDeserializer<'a, 'input> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Default to string for any
        if let Some(text) = self.node.text() {
            visitor.visit_string(text.to_string())
        } else {
            visitor.visit_string(String::new())
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        let b = text
            .parse::<bool>()
            .map_err(|_| Error::Custom(format!("invalid bool: {}", text)))?;
        visitor.visit_bool(b)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        let n = text
            .parse::<i8>()
            .map_err(|_| Error::Custom(format!("invalid i8: {}", text)))?;
        visitor.visit_i8(n)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        let n = text
            .parse::<i16>()
            .map_err(|_| Error::Custom(format!("invalid i16: {}", text)))?;
        visitor.visit_i16(n)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        let n = text
            .parse::<i32>()
            .map_err(|_| Error::Custom(format!("invalid i32: {}", text)))?;
        visitor.visit_i32(n)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        let n = text
            .parse::<i64>()
            .map_err(|_| Error::Custom(format!("invalid i64: {}", text)))?;
        visitor.visit_i64(n)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        let n = text
            .parse::<u8>()
            .map_err(|_| Error::Custom(format!("invalid u8: {}", text)))?;
        visitor.visit_u8(n)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        let n = text
            .parse::<u16>()
            .map_err(|_| Error::Custom(format!("invalid u16: {}", text)))?;
        visitor.visit_u16(n)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        let n = text
            .parse::<u32>()
            .map_err(|_| Error::Custom(format!("invalid u32: {}", text)))?;
        visitor.visit_u32(n)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        let n = text
            .parse::<u64>()
            .map_err(|_| Error::Custom(format!("invalid u64: {}", text)))?;
        visitor.visit_u64(n)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        let n = text
            .parse::<f32>()
            .map_err(|_| Error::Custom(format!("invalid f32: {}", text)))?;
        visitor.visit_f32(n)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        let n = text
            .parse::<f64>()
            .map_err(|_| Error::Custom(format!("invalid f64: {}", text)))?;
        visitor.visit_f64(n)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        let c = text.chars().next().ok_or_else(|| {
            Error::Custom("empty string for char".to_string())
        })?;
        visitor.visit_char(c)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        visitor.visit_string(text.to_string())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let text = self.node.text().unwrap_or("");
        visitor.visit_string(text.to_string())
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("bytes not supported".to_string()))
    }

    fn deserialize_byte_buf<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("byte_buf not supported".to_string()))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom(
            "seq deserialization requires xpath context".to_string(),
        ))
    }

    fn deserialize_tuple<V>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("tuple not supported".to_string()))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("tuple struct not supported".to_string()))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("map not supported".to_string()))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom(
            "struct deserialization requires xpath descriptors".to_string(),
        ))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("enum not supported".to_string()))
    }

    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

// Deserializer for attribute values
pub struct AttributeDeserializer<'a> {
    value: &'a str,
}

impl<'a> AttributeDeserializer<'a> {
    pub fn new(value: &'a str) -> Self {
        AttributeDeserializer { value }
    }
}

impl<'de, 'a> de::Deserializer<'de> for AttributeDeserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.value.to_string())
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let b = self.value.parse::<bool>().map_err(|_| {
            Error::Custom(format!("invalid bool: {}", self.value))
        })?;
        visitor.visit_bool(b)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.value.parse::<i8>().map_err(|_| {
            Error::Custom(format!("invalid i8: {}", self.value))
        })?;
        visitor.visit_i8(n)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.value.parse::<i16>().map_err(|_| {
            Error::Custom(format!("invalid i16: {}", self.value))
        })?;
        visitor.visit_i16(n)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.value.parse::<i32>().map_err(|_| {
            Error::Custom(format!("invalid i32: {}", self.value))
        })?;
        visitor.visit_i32(n)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.value.parse::<i64>().map_err(|_| {
            Error::Custom(format!("invalid i64: {}", self.value))
        })?;
        visitor.visit_i64(n)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.value.parse::<u8>().map_err(|_| {
            Error::Custom(format!("invalid u8: {}", self.value))
        })?;
        visitor.visit_u8(n)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.value.parse::<u16>().map_err(|_| {
            Error::Custom(format!("invalid u16: {}", self.value))
        })?;
        visitor.visit_u16(n)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.value.parse::<u32>().map_err(|_| {
            Error::Custom(format!("invalid u32: {}", self.value))
        })?;
        visitor.visit_u32(n)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.value.parse::<u64>().map_err(|_| {
            Error::Custom(format!("invalid u64: {}", self.value))
        })?;
        visitor.visit_u64(n)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.value.parse::<f32>().map_err(|_| {
            Error::Custom(format!("invalid f32: {}", self.value))
        })?;
        visitor.visit_f32(n)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.value.parse::<f64>().map_err(|_| {
            Error::Custom(format!("invalid f64: {}", self.value))
        })?;
        visitor.visit_f64(n)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let c = self.value.chars().next().ok_or_else(|| {
            Error::Custom("empty string for char".to_string())
        })?;
        visitor.visit_char(c)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.value.to_string())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.value.to_string())
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("bytes not supported".to_string()))
    }

    fn deserialize_byte_buf<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("byte_buf not supported".to_string()))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("seq not supported for attributes".to_string()))
    }

    fn deserialize_tuple<V>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("tuple not supported".to_string()))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("tuple struct not supported".to_string()))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("map not supported".to_string()))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("struct not supported for attributes".to_string()))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("enum not supported".to_string()))
    }

    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

// Deserializer for text content
pub struct TextDeserializer<'a> {
    text: &'a str,
}

impl<'a> TextDeserializer<'a> {
    pub fn new(text: &'a str) -> Self {
        TextDeserializer { text }
    }
}

impl<'de, 'a> de::Deserializer<'de> for TextDeserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.text.to_string())
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let b = self.text.parse::<bool>().map_err(|_| {
            Error::Custom(format!("invalid bool: {}", self.text))
        })?;
        visitor.visit_bool(b)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self
            .text
            .parse::<i8>()
            .map_err(|_| Error::Custom(format!("invalid i8: {}", self.text)))?;
        visitor.visit_i8(n)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.text.parse::<i16>().map_err(|_| {
            Error::Custom(format!("invalid i16: {}", self.text))
        })?;
        visitor.visit_i16(n)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.text.parse::<i32>().map_err(|_| {
            Error::Custom(format!("invalid i32: {}", self.text))
        })?;
        visitor.visit_i32(n)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.text.parse::<i64>().map_err(|_| {
            Error::Custom(format!("invalid i64: {}", self.text))
        })?;
        visitor.visit_i64(n)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self
            .text
            .parse::<u8>()
            .map_err(|_| Error::Custom(format!("invalid u8: {}", self.text)))?;
        visitor.visit_u8(n)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.text.parse::<u16>().map_err(|_| {
            Error::Custom(format!("invalid u16: {}", self.text))
        })?;
        visitor.visit_u16(n)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.text.parse::<u32>().map_err(|_| {
            Error::Custom(format!("invalid u32: {}", self.text))
        })?;
        visitor.visit_u32(n)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.text.parse::<u64>().map_err(|_| {
            Error::Custom(format!("invalid u64: {}", self.text))
        })?;
        visitor.visit_u64(n)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.text.parse::<f32>().map_err(|_| {
            Error::Custom(format!("invalid f32: {}", self.text))
        })?;
        visitor.visit_f32(n)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let n = self.text.parse::<f64>().map_err(|_| {
            Error::Custom(format!("invalid f64: {}", self.text))
        })?;
        visitor.visit_f64(n)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let c = self.text.chars().next().ok_or_else(|| {
            Error::Custom("empty string for char".to_string())
        })?;
        visitor.visit_char(c)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.text.to_string())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.text.to_string())
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("bytes not supported".to_string()))
    }

    fn deserialize_byte_buf<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("byte_buf not supported".to_string()))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("seq not supported for text".to_string()))
    }

    fn deserialize_tuple<V>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("tuple not supported".to_string()))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("tuple struct not supported".to_string()))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("map not supported".to_string()))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("struct not supported for text".to_string()))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("enum not supported".to_string()))
    }

    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

// Private module for struct descriptors
pub mod __private {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub enum FieldKind {
        Element,
        Text,
        Attribute,
        Sequence,
        Optional,
        OptionalSequence,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct FieldDescriptor {
        pub name: &'static str,
        pub xpath: &'static str,
        pub kind: FieldKind,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct StructDescriptor {
        pub name: &'static str,
        pub root_xpath: &'static str,
        pub fields: &'static [FieldDescriptor],
    }

    pub fn deserialize_xpath_struct<T, F>(
        xml: &str,
        descriptor: &StructDescriptor,
        field_deserializer: F,
    ) -> Result<T, Error>
    where
        F: FnOnce(StructFieldDeserializer<'_, '_>) -> Result<T, Error>,
    {
        let doc = roxmltree::Document::parse(xml)?;
        let root = doc.root_element();

        // Find the root node using the struct's xpath
        let root_xpath =
            XPath::parse(descriptor.root_xpath).map_err(Error::XPath)?;

        let target_node = root_xpath
            .evaluate_single(root)
            .and_then(|r| r.as_node())
            .ok_or_else(|| {
                Error::XPath(format!(
                    "root xpath '{}' not found",
                    descriptor.root_xpath
                ))
            })?;

        let deser = StructFieldDeserializer { node: target_node, descriptor };

        field_deserializer(deser)
    }

    pub struct StructFieldDeserializer<'a, 'input> {
        pub node: Node<'a, 'input>,
        pub descriptor: &'a StructDescriptor,
    }

    impl<'a, 'input> StructFieldDeserializer<'a, 'input> {
        pub fn deserialize_field<'de, T: serde::Deserialize<'de>>(
            &self,
            field_name: &str,
        ) -> Result<T, Error> {
            let field = self
                .descriptor
                .fields
                .iter()
                .find(|f| f.name == field_name)
                .ok_or_else(|| Error::MissingField(field_name.to_string()))?;

            let xpath = XPath::parse(field.xpath).map_err(Error::XPath)?;

            match field.kind {
                FieldKind::Attribute => {
                    let result =
                        xpath.evaluate_single(self.node).ok_or_else(|| {
                            Error::MissingField(field_name.to_string())
                        })?;

                    let value = result.as_str().ok_or_else(|| {
                        Error::XPath(format!(
                            "expected attribute for field '{}'",
                            field_name
                        ))
                    })?;

                    T::deserialize(AttributeDeserializer::new(value))
                }
                FieldKind::Text => {
                    let result =
                        xpath.evaluate_single(self.node).ok_or_else(|| {
                            Error::MissingField(field_name.to_string())
                        })?;

                    let text = result.text().ok_or_else(|| {
                        Error::XPath(format!(
                            "no text content for field '{}'",
                            field_name
                        ))
                    })?;

                    T::deserialize(TextDeserializer::new(text))
                }
                FieldKind::Element => {
                    let result =
                        xpath.evaluate_single(self.node).ok_or_else(|| {
                            Error::MissingField(field_name.to_string())
                        })?;

                    let node = result.as_node().ok_or_else(|| {
                        Error::XPath(format!(
                            "expected element for field '{}'",
                            field_name
                        ))
                    })?;

                    T::deserialize(XPathDeserializer::new(node))
                }
                FieldKind::Optional => {
                    // Return None if xpath doesn't match
                    match xpath.evaluate_single(self.node) {
                        Some(_) => {
                            // The Option<T> will be deserialized, we need to
                            // handle this carefully
                            // For now, just propagate
                            self.deserialize_field_inner::<T>(field)
                        }
                        None => {
                            // Return default (None for Option)
                            T::deserialize(OptionNoneDeserializer)
                        }
                    }
                }
                FieldKind::Sequence => {
                    let results = xpath.evaluate_all(self.node);
                    let nodes: Vec<_> =
                        results.iter().filter_map(|r| r.as_node()).collect();
                    T::deserialize(SeqDeserializer::new(nodes))
                }
                FieldKind::OptionalSequence => {
                    let results = xpath.evaluate_all(self.node);
                    let nodes: Vec<_> =
                        results.iter().filter_map(|r| r.as_node()).collect();
                    T::deserialize(SeqDeserializer::new(nodes))
                }
            }
        }

        fn deserialize_field_inner<'de, T: serde::Deserialize<'de>>(
            &self,
            field: &FieldDescriptor,
        ) -> Result<T, Error> {
            let xpath = XPath::parse(field.xpath).map_err(Error::XPath)?;

            let result = xpath
                .evaluate_single(self.node)
                .ok_or_else(|| Error::MissingField(field.name.to_string()))?;

            match result {
                XPathResult::Node(node) => {
                    T::deserialize(XPathDeserializer::new(node))
                }
                XPathResult::Attribute(value) => {
                    T::deserialize(AttributeDeserializer::new(value))
                }
            }
        }

        pub fn node(&self) -> Node<'a, 'input> {
            self.node
        }
    }

    // Deserializer that always returns None
    pub struct OptionNoneDeserializer;

    impl<'de> de::Deserializer<'de> for OptionNoneDeserializer {
        type Error = Error;

        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
            byte_buf unit unit_struct newtype_struct seq tuple tuple_struct
            map struct enum identifier ignored_any
        }

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_none()
        }

        fn deserialize_option<V>(
            self,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_none()
        }
    }

    // Sequence deserializer for Vec fields
    pub struct SeqDeserializer<'a, 'input> {
        nodes: Vec<Node<'a, 'input>>,
    }

    impl<'a, 'input> SeqDeserializer<'a, 'input> {
        pub fn new(nodes: Vec<Node<'a, 'input>>) -> Self {
            SeqDeserializer { nodes }
        }
    }

    impl<'de, 'a, 'input> de::Deserializer<'de> for SeqDeserializer<'a, 'input> {
        type Error = Error;

        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
            byte_buf option unit unit_struct newtype_struct tuple tuple_struct
            map struct enum identifier ignored_any
        }

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.deserialize_seq(visitor)
        }

        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_seq(SeqAccess { nodes: self.nodes, index: 0 })
        }
    }

    struct SeqAccess<'a, 'input> {
        nodes: Vec<Node<'a, 'input>>,
        index: usize,
    }

    impl<'de, 'a, 'input> de::SeqAccess<'de> for SeqAccess<'a, 'input> {
        type Error = Error;

        fn next_element_seed<T>(
            &mut self,
            seed: T,
        ) -> Result<Option<T::Value>, Self::Error>
        where
            T: de::DeserializeSeed<'de>,
        {
            if self.index >= self.nodes.len() {
                return Ok(None);
            }

            let node = self.nodes[self.index];
            self.index += 1;

            let value = seed.deserialize(XPathDeserializer::new(node))?;
            Ok(Some(value))
        }
    }

    // Helper function for deserializing from a node with a custom struct
    // descriptor
    pub fn deserialize_struct_from_node<'de, 'a, 'input, T, F>(
        node: Node<'a, 'input>,
        descriptor: &'static StructDescriptor,
        field_deserializer: F,
    ) -> Result<T, Error>
    where
        F: FnOnce(StructFieldDeserializer<'a, 'input>) -> Result<T, Error>,
    {
        let deser = StructFieldDeserializer { node, descriptor };
        field_deserializer(deser)
    }
}
