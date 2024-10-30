use core::fmt::Display;
use core::ops::{Deref, DerefMut};

use serde::{serde_if_integer128, Serialize, Serializer};

/// A newtype implementing [`serde::Deserializer`] if the type `D` implements [`AsTransientDeserializer`].
#[derive(Debug, Clone, Copy)]
pub struct PersistentSerializer<D>(D);

impl<'a, D> PersistentSerializer<D> {
    pub fn new(serializer: D) -> Self {
        Self(serializer)
    }
}
/*
impl<D> Deref for PersistentSerializer<D> {
    type Target = D;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<D> DerefMut for PersistentSerializer<D> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
*/
pub trait AsTransientSerializer {
    type Ok;
    type Error: serde::ser::Error;

    fn as_transient_serializer<'a>(&'a mut self) -> impl Serializer<
        Ok=Self::Ok,
        Error=Self::Error,
    >;

    fn as_transient_serialize_seq<'a>(&'a mut self) -> impl serde::ser::SerializeSeq<
        Ok=Self::Ok,
        Error=Self::Error,
    >;

    fn is_human_readable(&self) -> bool;
}

struct PersistentSerializeSeq<D>(D);

impl<S: AsTransientSerializer> serde::ser::SerializeSeq for PersistentSerializer<S> {
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error> where T: ?Sized + Serialize {
        self.0.as_transient_serialize_seq().serialize_element(value)
    }
    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serialize_seq().end()
    }
}


impl<S: AsTransientSerializer> Serializer for PersistentSerializer<S> {
    type Ok = S::Ok;
    type Error = S::Error;

    type SerializeSeq = Self;

    /*
    type SerializeSeq = S::SerializeSeq;
    type SerializeTuple = S::SerializeTuple;
    type SerializeTupleStruct = S::SerializeTupleStruct;
    type SerializeTupleVariant = S::SerializeTupleVariant;
    type SerializeMap = S::SerializeMap;
    type SerializeStruct = S::SerializeStruct;
    type SerializeStructVariant = S::SerializeStructVariant;
    */

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_bool(v)
    }
    fn serialize_i8(mut self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_i8(v)
    }
    fn serialize_i16(mut self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_i16(v)
    }
    fn serialize_i32(mut self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_i32(v)
    }
    fn serialize_i64(mut self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_i64(v)
    }
    serde_if_integer128! {
        fn serialize_i128(mut self, v: i128) -> Result<Self::Ok, Self::Error> {
            self.0.as_transient_serializer().serialize_i128(v)
        }
    }
    fn serialize_u8(mut self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_u8(v)
    }
    fn serialize_u16(mut self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_u16(v)
    }
    fn serialize_u32(mut self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_u32(v)
    }
    fn serialize_u64(mut self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_u64(v)
    }
    serde_if_integer128! {
        fn serialize_u128(mut self, v: u128) -> Result<Self::Ok, Self::Error> {
            self.0.as_transient_serializer().serialize_u128(v)
        }
    }
    fn serialize_f32(mut self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_f32(v)
    }
    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_f64(v)
    }
    fn serialize_char(mut self, v: char) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_char(v)
    }
    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_str(v)
    }
    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_bytes(v)
    }
    fn serialize_none(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_none()
    }
    fn serialize_some<T>(mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.as_transient_serializer().serialize_some(value)
    }
    fn serialize_unit(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_unit()
    }
    fn serialize_unit_struct(mut self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_unit_struct(name)
    }
    fn serialize_unit_variant(
        mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.0
            .as_transient_serializer()
            .serialize_unit_variant(name, variant_index, variant)
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        mut self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        self.0
            .as_transient_serializer()
            .serialize_newtype_struct(name, value)
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        self.0.as_transient_serializer().serialize_newtype_variant(
            name,
            variant_index,
            variant,
            value,
        )
    }
    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.0.as_transient_serializer().serialize_seq(len)
    }
    fn serialize_tuple(mut self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.0.as_transient_serializer().serialize_tuple(len)
    }
    fn serialize_tuple_struct(
        mut self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.0
            .as_transient_serializer()
            .serialize_tuple_struct(name, len)
    }
    fn serialize_tuple_variant(
        mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.0
            .as_transient_serializer()
            .serialize_tuple_variant(name, variant_index, variant, len)
    }
    fn serialize_map(mut self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.0.as_transient_serializer().serialize_map(len)
    }
    fn serialize_struct(
        mut self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.0.as_transient_serializer().serialize_struct(name, len)
    }
    fn serialize_struct_variant(
        mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.0
            .as_transient_serializer()
            .serialize_struct_variant(name, variant_index, variant, len)
    }
    fn collect_seq<I>(mut self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        self.0.as_transient_serializer().collect_seq(iter)
    }
    fn collect_map<K, V, I>(mut self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        self.0.as_transient_serializer().collect_map(iter)
    }
    fn collect_str<T>(mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        self.0.as_transient_serializer().collect_str(value)
    }
    fn is_human_readable(&self) -> bool {
        self.0.is_human_readable()
    }
}
