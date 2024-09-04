//! Procedural macros for `rkyv`.

#![deny(
    rustdoc::broken_intra_doc_links,
    missing_docs,
    rustdoc::missing_crate_level_docs
)]

mod archive;
mod attributes;
mod deserialize;
mod portable;
mod repr;
mod serde;
mod serialize;
mod util;

extern crate proc_macro;

use syn::{parse_macro_input, DeriveInput};

/// Derives `Portable` for the labeled type.
///
/// This macro also supports the `#[omit_bounds]` attribute. See [`Archive`] for
/// more information.
#[proc_macro_derive(Portable, attributes(archive, rkyv, omit_bounds))]
pub fn derive_portable(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut derive_input = parse_macro_input!(input as DeriveInput);
    serde::receiver::replace_receiver(&mut derive_input);

    match portable::derive(derive_input) {
        Ok(result) => result.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Derives `Archive` for the labeled type.
///
/// # Attributes
///
/// Additional arguments can be specified using attributes.
///
/// `#[rkyv(...)]` accepts the following arguments:
///
/// - `archived = ...`: Changes the name of the generated archived type to the
///   given value. By default, archived types are named "Archived" + `the name
///   of the type`.
/// - `resolver = ...`: Changes the name of the generated resolver type to the
///   given value. By default, resolver types are named `the name of the type` +
///   "Resolver".
/// - `attr(...)`: Passes along attributes to the generated archived type such
///   as `repr(...)`.
/// - `derive(...)`: Adds the derives passed as arguments to the generated type.
/// - `compare(...)`: Implements common comparison operators between the
///   original and archived types. Supported comparisons are `PartialEq` and
///   `PartialOrd` (i.e. `#[rkyv(compare(PartialEq, PartialOrd))]`).
/// - `{archive, serialize, deserialize}_bounds(...)`: Adds additional bounds to
///   trait implementations. This can be especially useful when dealing with
///   recursive structures, where bounds may need to be omitted to prevent
///   recursive type definitions.
/// - `check_bytes`: Derive `CheckBytes` on the archived type, in order to
///   enable safe deserialization. Requires `bytecheck` feature. Not compatible
///   with `as = ...`. In that case, use `#[derive(CheckBytes)]` and
///   `#[check_bytes(crate = rkyv::bytecheck)]` on the archived type.
/// - `as = ...`: Instead of generating a separate archived type, this type will
///   archive as the named type. This is useful for types which are generic over
///   their parameters.
/// - `crate = ...`: Chooses an alternative crate path to import rkyv from.
///
/// There are also shorthand attributes:
///
/// - `#[rkyv_attr(...)]` is shorthand for `#[rkyv(attr(...))]`.
/// - `#[rkyv_derive(...)]` is shorthand for `#[rkyv(derive(...))]`.
///
/// # Recursive types
///
/// This derive macro automatically adds a type bound `field: Archive` for each
/// field type. This can cause an overflow while evaluating trait bounds if the
/// structure eventually references its own type, as the implementation of
/// `Archive` for a struct depends on each field type implementing it
/// as well. Adding the attribute `#[omit_bounds]` to a field will suppress this
/// trait bound and allow recursive structures. This may be too coarse for some
/// types, in which case additional type bounds may be required with
/// `{archive, serialize, deserialize}_bounds(...)`.
///
/// # Wrappers
///
/// Wrappers transparently customize archived types by providing different
/// implementations of core traits. For example, references cannot be archived,
/// but the `Inline` wrapper serializes a reference as if it were a field of the
/// struct. Wrappers can be applied to fields using the `#[with(...)]`
/// attribute.
#[proc_macro_derive(
    Archive,
    attributes(
        archive,
        rkyv,
        archive_attr,
        rkyv_attr,
        rkyv_derive,
        omit_bounds,
        with
    )
)]
pub fn derive_archive(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut derive_input = parse_macro_input!(input as DeriveInput);
    serde::receiver::replace_receiver(&mut derive_input);

    match archive::derive(&mut derive_input) {
        Ok(result) => result.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Derives `Serialize` for the labeled type.
///
/// This macro also supports the `#[rkyv]`, `#[omit_bounds]`, and `#[with]`
/// attributes. See [`Archive`] for more information.
#[proc_macro_derive(Serialize, attributes(archive, rkyv, omit_bounds, with))]
pub fn derive_serialize(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut derive_input = parse_macro_input!(input as DeriveInput);
    serde::receiver::replace_receiver(&mut derive_input);

    match serialize::derive(derive_input) {
        Ok(result) => result.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Derives `Deserialize` for the labeled type.
///
/// This macro also supports the `#[rkyv]`, `#[omit_bounds]`, and `#[with]`
/// attributes. See [`Archive`] for more information.
#[proc_macro_derive(Deserialize, attributes(archive, rkyv, omit_bounds, with))]
pub fn derive_deserialize(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut derive_input = parse_macro_input!(input as DeriveInput);
    serde::receiver::replace_receiver(&mut derive_input);

    match deserialize::derive(derive_input) {
        Ok(result) => result.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
