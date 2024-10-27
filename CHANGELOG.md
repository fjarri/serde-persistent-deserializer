# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-10-27

- MSRV bumped to 1.75.0 (because of `-> impl Trait` in a trait method).
- Renamed `AsMutDeserializer` -> `AsTransientDeserializer`.
- Added `Deref` and `DerefMut` impls for `PersistentDeserializer`.


## [0.1.0] - 2024-10-27


[0.1.0]: https://github.com/fjarri/serde-persistent-deserializer/releases/tag/v0.1.0
