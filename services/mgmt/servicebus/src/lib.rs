#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2021-01-preview")]
pub mod package_2021_01_preview;
#[cfg(all(feature = "package-2021-01-preview", not(feature = "no-default-version")))]
pub use package_2021_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-06-preview")]
pub mod package_2021_06_preview;
#[cfg(all(feature = "package-2021-06-preview", not(feature = "no-default-version")))]
pub use package_2021_06_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-01-preview")]
pub mod package_2018_01_preview;
#[cfg(all(feature = "package-2018-01-preview", not(feature = "no-default-version")))]
pub use package_2018_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-11")]
pub mod package_2021_11;
#[cfg(all(feature = "package-2021-11", not(feature = "no-default-version")))]
pub use package_2021_11::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2017-04")]
pub mod package_2017_04;
#[cfg(all(feature = "package-2017-04", not(feature = "no-default-version")))]
pub use package_2017_04::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2015-08")]
pub mod package_2015_08;
#[cfg(all(feature = "package-2015-08", not(feature = "no-default-version")))]
pub use package_2015_08::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
