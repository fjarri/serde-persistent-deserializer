# A wrapper for persistent `serde` deserializers

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![License][license-image]
[![Build Status][build-image]][build-link]
[![Coverage][coverage-image]][coverage-link]

When one writes an implementation of [`serde::Deserializer`](https://docs.rs/serde/latest/serde/trait.Deserializer.html) for their format, it is commonly written for a `&mut` of some type, or a wrapper of it --- because it has to be recursively passed down when deserializing nested structs, lists, and so on.

But if someone else wants to be generic over types that implement `Deserializer` (e.g. when using the [`erased-serde`](https://docs.rs/erased-serde) crate), it necessarily produces higher-ranked bounds on lifetimes (something along the lines of `for<'a, 'de> &'a mut D: Deserializer<'de>`), which cannot be encapsulated in a trait and will be propagated to all dependent generic code (a current limitation of Rust; see <https://github.com/rust-lang/rust/issues/50346> and other related issues). Also, using [`erased_serde::Deserializer::erase`](https://docs.rs/erased-serde/latest/erased_serde/trait.Deserializer.html#method.erase) on [`serde::Deserializer`](https://docs.rs/serde/latest/serde/trait.Deserializer.html) types in a real world code is not trivial (see <https://github.com/dtolnay/erased-serde/issues/107> for an example).

To amend that, it would be convenient to additionally implement [`serde::Deserializer`](https://docs.rs/serde/latest/serde/trait.Deserializer.html) on the object itself, which involves writing a long sheet of boilerplate like
```ignore
struct MyDeserializerRef<'a, 'de> {
    de: &'a mut MyDeserializer<'de>
}

impl<'a, 'de> Deserializer<'de> for MyDeserializerRef<'a, 'de> {
    // ... actual deserialization logic here
}

impl<'de> MyDeserializer<'de> {
    fn as_mut_deserializer<'a>(&'a mut self) -> MyDeserializerRef<'a, 'de> { ... }
}

impl<'de> Deserializer<'de> for MyDeserializer<'de> {
    fn deserialize_any<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    {
        self.as_mut_deserializer().deserialize_any(visitor)
    }

    fn deserialize_bool<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    {
        self.as_mut_deserializer().deserialize_bool(visitor)
    }

    // ... and dozens more methods with the same content
}
```

This crate instead requires one to only provide an implementation of [`AsMutDeserializer`] for their type, and then the [`PersistentDeserializer`] wrapper will automatically derive [`serde::Deserializer`](https://docs.rs/serde/latest/serde/trait.Deserializer.html).


[crate-image]: https://img.shields.io/crates/v/serde-persistent-deserializer.svg
[crate-link]: https://crates.io/crates/serde-persistent-deserializer
[docs-image]: https://docs.rs/serde-persistent-deserializer/badge.svg
[docs-link]: https://docs.rs/serde-persistent-deserializer/
[license-image]: https://img.shields.io/crates/l/serde-persistent-deserializer
[build-image]: https://github.com/fjarri/serde-persistent-deserializer/actions/workflows/ci.yml/badge.svg?branch=master&event=push
[build-link]: https://github.com/fjarri/serde-persistent-deserializer/actions?query=workflow%3Aci
[coverage-image]: https://codecov.io/gh/fjarri/serde-persistent-deserializer/branch/master/graph/badge.svg
[coverage-link]: https://codecov.io/gh/fjarri/serde-persistent-deserializer
