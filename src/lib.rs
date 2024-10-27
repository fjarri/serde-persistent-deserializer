#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![warn(
    clippy::mod_module_files,
    clippy::unwrap_used,
    clippy::indexing_slicing,
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications
)]

use serde::{de::Visitor, serde_if_integer128, Deserializer};

/// A newtype implementing [`serde::Deserializer`] if the type `D` implements [`AsMutDeserializer`].
#[derive(Debug, Clone, Copy)]
pub struct PersistentDeserializer<D>(D);

impl<D> PersistentDeserializer<D> {
    /// Wraps the deserializer object.
    pub fn new(deserializer: D) -> Self {
        Self(deserializer)
    }
}

/// Describes a type that can produce a [`serde::Deserializer`]-implementing object.
pub trait AsMutDeserializer<'de> {
    /// The deserialization error type.
    type Error: serde::de::Error;

    /// Produces a deserializer object.
    fn as_mut_deserializer<'a>(&'a mut self) -> impl Deserializer<'de, Error = Self::Error>;

    /// Determine whether `Deserialize` implementations should expect to deserialize
    /// their human-readable form.
    ///
    /// This method will be called in the `Deserializer::is_human_readable` impl for
    /// [`PersistentDeserializer`].
    fn is_human_readable(&self) -> bool;
}

impl<'de, D: AsMutDeserializer<'de>> Deserializer<'de> for PersistentDeserializer<D> {
    type Error = <D as AsMutDeserializer<'de>>::Error;

    fn deserialize_any<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_any(visitor)
    }

    fn deserialize_bool<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_bool(visitor)
    }

    fn deserialize_i8<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_i8(visitor)
    }

    fn deserialize_i16<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_i16(visitor)
    }

    fn deserialize_i32<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_i32(visitor)
    }

    fn deserialize_i64<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_i64(visitor)
    }

    serde_if_integer128! {
        fn deserialize_i128<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        {
            self.0.as_mut_deserializer().deserialize_i128(visitor)
        }
    }

    fn deserialize_u8<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_u8(visitor)
    }

    fn deserialize_u16<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_u16(visitor)
    }

    fn deserialize_u32<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_u32(visitor)
    }

    fn deserialize_u64<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_u64(visitor)
    }

    serde_if_integer128! {
        fn deserialize_u128<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        {
            self.0.as_mut_deserializer().deserialize_u128(visitor)
        }
    }

    fn deserialize_f32<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_f32(visitor)
    }

    fn deserialize_f64<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_f64(visitor)
    }

    fn deserialize_char<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_char(visitor)
    }

    fn deserialize_str<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_str(visitor)
    }

    #[cfg(not(feature = "alloc"))]
    fn deserialize_string<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_string(visitor)
    }

    #[cfg(feature = "alloc")]
    fn deserialize_string<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_string(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_bytes(visitor)
    }

    #[cfg(not(feature = "alloc"))]
    fn deserialize_byte_buf<V: Visitor<'de>>(
        mut self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_byte_buf(visitor)
    }

    #[cfg(feature = "alloc")]
    fn deserialize_byte_buf<V: Visitor<'de>>(
        mut self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_byte_buf(visitor)
    }

    fn deserialize_option<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_option(visitor)
    }

    fn deserialize_unit<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_unit(visitor)
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_mut_deserializer()
            .deserialize_unit_struct(name, visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_mut_deserializer()
            .deserialize_newtype_struct(name, visitor)
    }

    fn deserialize_seq<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_seq(visitor)
    }

    fn deserialize_tuple<V: Visitor<'de>>(
        mut self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_mut_deserializer()
            .deserialize_tuple_struct(name, len, visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_map(visitor)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_mut_deserializer()
            .deserialize_struct(name, fields, visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_mut_deserializer()
            .deserialize_enum(name, variants, visitor)
    }

    fn deserialize_identifier<V: Visitor<'de>>(
        mut self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.as_mut_deserializer().deserialize_identifier(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(
        mut self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_mut_deserializer()
            .deserialize_ignored_any(visitor)
    }

    fn is_human_readable(&self) -> bool {
        self.0.is_human_readable()
    }
}
