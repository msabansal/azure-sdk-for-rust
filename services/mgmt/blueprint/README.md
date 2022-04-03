# azure_mgmt_blueprint crate

The is an [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust) crate that is generated from the Azure REST API specifications listed in:

https://github.com/Azure/azure-rest-api-specs/blob/main/specification/blueprint/resource-manager/readme.md

The default `Tag` is `package-2018-11-preview`.

The following `Tag`s are available:

- `package-2017-11-preview` has 19 operations from 1 API versions: `2017-11-11-preview`. Use crate feature `package-2017-11-preview` to enable. The operations will be in the `package_2017_11_preview` module.
- `package-2018-11-preview` has 21 operations from 1 API versions: `2018-11-01-preview`. Use crate feature `package-2018-11-preview` to enable. The operations will be in the `package_2018_11_preview` module.