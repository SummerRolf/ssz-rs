# ssz-rs ✂️️

[![build](https://github.com/ralexstokes/ssz-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/ralexstokes/ssz-rs/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/ssz_rs.svg)](https://crates.io/crates/ssz_rs)
[![docs.rs](https://img.shields.io/docsrs/ssz_rs)](https://docs.rs/ssz_rs/)

An implementation of the `SSZ` serialization scheme defined in the [consensus-specs repo](https://github.com/ethereum/consensus-specs).

This repo aims to remain lightweight and relatively free-standing, rather than coupled to other ethereum consensus code/dependencies.

# 🚧 WARNING 🚧

This implementation has **not** been audited for security and is primarily intended for R&D use cases.

If you need a battle-tested implementation (e.g. for consensus-critical work), refer to the [Lighthouse implementation](https://github.com/sigp/lighthouse).

# Features

To conform to the `SSZ` spec, a given Rust type should implement the [`SimpleSerialize` trait](https://docs.rs/ssz_rs/latest/ssz_rs/trait.SimpleSerialize.html). Types implementing this trait then obtain:

## Encoding / decoding

`ssz_rs` aims to add as little ceremony over the built-in Rust types as possible.
The `ssz_rs_derive` crate provides macros to derive the encoding and decoding routines for SSZ containers and unions (represented as Rust `struct`s and `enum`s, respectively).
See the `ssz_rs/examples` for example usage.

## Merkleization

This library provides the hash tree root computation for types implementing `SimpleSerialize`.

* *NOTE*: more sophisticated caching strategies are possible, users may run into memory issues with the current implementation.

## Multiproofs

This library provides the ability to reason about [generalized indices](https://github.com/ethereum/consensus-specs/blob/dev/ssz/merkle-proofs.md#generalized-merkle-tree-index) for a given `SSZ` definition,
along with the ability to generate and verify proofs of data at those indices.

* *NOTE*: still under construction

## `no-std` feature

This library is `no-std` compatible. To build without the standard library, disable the crate's default features.

For example, in `Cargo.toml`:

```toml
ssz_rs = { version = "...", default-features = false }
```

# Testing

This repo includes a copy of the [`ssz_generic` consensus spec tests](https://github.com/ethereum/consensus-spec-tests) as integration tests for the `ssz_rs` crate, along with hand-written unit tests.
The integration tests are generated via a utility under `ssz-rs-test-gen` package. See the README there for further details.
