#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2021-10-31-preview")]
pub mod package_2021_10_31_preview;
#[cfg(all(feature = "package-2021-10-31-preview", not(feature = "no-default-version")))]
pub use package_2021_10_31_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
