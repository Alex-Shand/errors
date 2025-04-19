//! Error managment macros
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

/// Generate error handling boilerplate using Snafu
///
/// # Meta Arguments
/// * `crate`: (Optional) Set the path to the [`errors`](crate) crate. Will be inferred
///   from `Cargo.toml` if not present.
/// * `wrapping`: (Optional) Error type this error wraps.
/// * `display`: (Required) Arguments for the generated
///   [`Display`](std::fmt::Display) impl. Format: `display("format string", args...)`.
/// * `context`: (Optional) Visibility modifier for Snafu context selectors.
/// * `backtrace`: (Optional) Backtrace source. Default if not present uses the
///   backtrace from the wrapped error, if there is no wrapped error then no
///   backtrace is captured. If present as `backtrace`, a new backtrace will be
///   captured at the point this error is generated. If present as
///   `backtrace(field)` then the backtrace is taken from the contents of `field`
/// * `test_whatever`: (Switch) See the option in [`errors::union`](crate::union)
///
/// # Item
/// `#[errors::error]` should be applied to a struct, tuple structs are not
/// supported. The field names `location`, `backtrace` and `source` are reserved
/// by the macro. Depending on options passed to the macro fields with these
/// names will be added to the struct. If the `test_whatever` option is passed
/// the struct will actually become an enum to accommodate the
/// [`Whatever`](snafu::Whatever) branch.
pub use errors_macros::error;
/// Generate a union of multiple other errors
///
/// # Meta Arguments
/// * `test_whatever`: (Switch) If present the generated enum will have an extra
///   variant suitable for use with [`whatever`](snafu::whatever).
///
/// # Item
/// `#[errors::union]` should be applied to a type alias for a tuple type
/// composed of simple types e.g `type MyError = (Error1, Error2, Error3);`. The
/// macro generates an enum with a variant for each error type (and one
/// [`Whatever`](snafu::Whatever)) variant if the `test_whatever` argument is
/// passed) as well as suitable context selectors and conversion operators.
pub use errors_macros::union;
pub use snafu::{self, prelude};
