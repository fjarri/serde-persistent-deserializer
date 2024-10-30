use core::ops::{Deref, DerefMut};

use serde::{de::Visitor, serde_if_integer128, Deserializer};

/// A newtype implementing [`serde::Deserializer`] if the type `D` implements [`AsTransientDeserializer`].
#[derive(Debug, Clone, Copy)]
pub struct PersistentDeserializer<D>(D);

impl<D> PersistentDeserializer<D> {
    /// Wraps the deserializer object.
    pub fn new(deserializer: D) -> Self {
        Self(deserializer)
    }
}

impl<D> Deref for PersistentDeserializer<D> {
    type Target = D;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<D> DerefMut for PersistentDeserializer<D> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Describes a type that can produce a [`serde::Deserializer`]-implementing object.
pub trait AsTransientDeserializer<'de> {
    /// The deserialization error type.
    type Error: serde::de::Error;

    /// Produces a deserializer object.
    fn as_transient_deserializer<'a>(&'a mut self) -> impl Deserializer<'de, Error = Self::Error>;

    /// Determine whether `Deserialize` implementations should expect to deserialize
    /// their human-readable form.
    ///
    /// This method will be called in the `Deserializer::is_human_readable` impl for
    /// [`PersistentDeserializer`].
    fn is_human_readable(&self) -> bool;
}

impl<'de, D: AsTransientDeserializer<'de>> Deserializer<'de> for PersistentDeserializer<D> {
    type Error = D::Error;

    #[inline]
    fn deserialize_any<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_any(visitor)
    }

    #[inline]
    fn deserialize_bool<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_bool(visitor)
    }

    #[inline]
    fn deserialize_i8<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_i8(visitor)
    }

    #[inline]
    fn deserialize_i16<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_i16(visitor)
    }

    #[inline]
    fn deserialize_i32<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_i32(visitor)
    }

    #[inline]
    fn deserialize_i64<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_i64(visitor)
    }

    serde_if_integer128! {
        #[inline]
        fn deserialize_i128<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        {
            self.0.as_transient_deserializer().deserialize_i128(visitor)
        }
    }

    #[inline]
    fn deserialize_u8<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_u8(visitor)
    }

    #[inline]
    fn deserialize_u16<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_u16(visitor)
    }

    #[inline]
    fn deserialize_u32<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_u32(visitor)
    }

    #[inline]
    fn deserialize_u64<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_u64(visitor)
    }

    serde_if_integer128! {
        #[inline]
        fn deserialize_u128<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        {
            self.0.as_transient_deserializer().deserialize_u128(visitor)
        }
    }

    #[inline]
    fn deserialize_f32<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_f32(visitor)
    }

    #[inline]
    fn deserialize_f64<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_f64(visitor)
    }

    #[inline]
    fn deserialize_char<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_char(visitor)
    }

    #[inline]
    fn deserialize_str<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_string<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0
            .as_transient_deserializer()
            .deserialize_string(visitor)
    }

    #[inline]
    fn deserialize_bytes<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0
            .as_transient_deserializer()
            .deserialize_bytes(visitor)
    }

    #[inline]
    fn deserialize_byte_buf<V: Visitor<'de>>(
        mut self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_transient_deserializer()
            .deserialize_byte_buf(visitor)
    }

    #[inline]
    fn deserialize_option<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0
            .as_transient_deserializer()
            .deserialize_option(visitor)
    }

    #[inline]
    fn deserialize_unit<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_unit(visitor)
    }

    #[inline]
    fn deserialize_unit_struct<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_transient_deserializer()
            .deserialize_unit_struct(name, visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_transient_deserializer()
            .deserialize_newtype_struct(name, visitor)
    }

    #[inline]
    fn deserialize_seq<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_tuple<V: Visitor<'de>>(
        mut self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_transient_deserializer()
            .deserialize_tuple(len, visitor)
    }

    #[inline]
    fn deserialize_tuple_struct<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_transient_deserializer()
            .deserialize_tuple_struct(name, len, visitor)
    }

    #[inline]
    fn deserialize_map<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.as_transient_deserializer().deserialize_map(visitor)
    }

    #[inline]
    fn deserialize_struct<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_transient_deserializer()
            .deserialize_struct(name, fields, visitor)
    }

    #[inline]
    fn deserialize_enum<V: Visitor<'de>>(
        mut self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_transient_deserializer()
            .deserialize_enum(name, variants, visitor)
    }

    #[inline]
    fn deserialize_identifier<V: Visitor<'de>>(
        mut self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_transient_deserializer()
            .deserialize_identifier(visitor)
    }

    #[inline]
    fn deserialize_ignored_any<V: Visitor<'de>>(
        mut self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .as_transient_deserializer()
            .deserialize_ignored_any(visitor)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.0.is_human_readable()
    }
}