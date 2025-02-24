# azure_mgmt_activedirectory crate

The is an [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust) crate that is generated from the Azure REST API specifications listed in:

https://github.com/Azure/azure-rest-api-specs/blob/main/specification/azureactivedirectory/resource-manager/readme.md

The default `Tag` is `package-2020-03`.

The following `Tag`s are available:

- `package-preview-2017-04` has 6 operations from 1 API versions: `2017-04-01-preview`. Use crate feature `package-preview-2017-04` to enable. The operations will be in the `package_preview_2017_04` module.
- `package-preview-2020-03` has 8 operations from 1 API versions: `2020-03-01-preview`. Use crate feature `package-preview-2020-03` to enable. The operations will be in the `package_preview_2020_03` module.
- `package-2020-03` has 12 operations from 1 API versions: `2020-03-01`. Use crate feature `package-2020-03` to enable. The operations will be in the `package_2020_03` module.
- `package-2017-04-01` has 6 operations from 1 API versions: `2017-04-01`. Use crate feature `package-2017-04-01` to enable. The operations will be in the `package_2017_04_01` module.