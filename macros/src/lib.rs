//! errors-macros
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_results)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::let_underscore_untyped)]
#![allow(clippy::similar_names)]

mod error;
mod from_ref;
mod union_;

// Documented in re-export from errors
#[allow(missing_docs)]
#[proc::attribute(host = "errors")]
pub fn error(
    crate_: proc::Path,
    wrapping: proc::meta::Optional<proc::syn::Type>,
    display: proc::meta::List<proc::syn::Expr>,
    context: proc::meta::Optional<proc::syn::Visibility>,
    backtrace: proc::meta::Custom<Option<error::BacktraceSource>>,
    test_whatever: proc::meta::Switch,
    input: proc::syn::ItemStruct,
) -> proc::Result<error::Error> {
    error::Error::new(
        crate_,
        wrapping,
        display,
        context,
        backtrace,
        test_whatever,
        input,
    )
}

// Documented in re-export from errors
#[allow(missing_docs)]
#[proc::attribute(host = "errors")]
pub fn union(
    crate_: proc::Path,
    test_whatever: proc::meta::Switch,
    input: proc::syn::ItemType,
) -> proc::Result<union_::Union> {
    union_::Union::new(crate_, test_whatever, input)
}

// Documented in re-export from errors
#[allow(missing_docs)]
#[proc::attribute]
pub fn from_ref(input: proc::DeriveInput) -> proc::Result<from_ref::FromRef> {
    Ok(from_ref::FromRef::new(input))
}
