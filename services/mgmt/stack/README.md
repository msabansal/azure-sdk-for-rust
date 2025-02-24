# azure_mgmt_stack crate

The is an [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust) crate that is generated from the Azure REST API specifications listed in:

https://github.com/Azure/azure-rest-api-specs/blob/main/specification/azurestack/resource-manager/readme.md

The default `Tag` is `package-preview-2020-06`.

The following `Tag`s are available:

- `package-2016-01` has 14 operations from 1 API versions: `2016-01-01`. Use crate feature `package-2016-01` to enable. The operations will be in the `package_2016_01` module.
- `package-2017-06-01` has 21 operations from 1 API versions: `2017-06-01`. Use crate feature `package-2017-06-01` to enable. The operations will be in the `package_2017_06_01` module.
- `package-preview-2020-06` has 27 operations from 1 API versions: `2020-06-01-preview`. Use crate feature `package-preview-2020-06` to enable. The operations will be in the `package_preview_2020_06` module.