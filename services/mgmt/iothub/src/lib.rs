#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub mod profile_hybrid_2020_09_01;
#[cfg(all(feature = "profile-hybrid-2020-09-01", not(feature = "no-default-tag")))]
pub use profile_hybrid_2020_09_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2021-07-02")]
pub mod package_preview_2021_07_02;
#[cfg(all(feature = "package-preview-2021-07-02", not(feature = "no-default-tag")))]
pub use package_preview_2021_07_02::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2021-03")]
pub mod package_preview_2021_03;
#[cfg(all(feature = "package-preview-2021-03", not(feature = "no-default-tag")))]
pub use package_preview_2021_03::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2021-02")]
pub mod package_preview_2021_02;
#[cfg(all(feature = "package-preview-2021-02", not(feature = "no-default-tag")))]
pub use package_preview_2021_02::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2020-08-31")]
pub mod package_preview_2020_08_31;
#[cfg(all(feature = "package-preview-2020-08-31", not(feature = "no-default-tag")))]
pub use package_preview_2020_08_31::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
