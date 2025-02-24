# azure_mgmt_advisor crate

The is an [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust) crate that is generated from the Azure REST API specifications listed in:

https://github.com/Azure/azure-rest-api-specs/blob/main/specification/advisor/resource-manager/readme.md

The default `Tag` is `package-2020-01`.

The following `Tag`s are available:

- `package-2020-07-preview` has 3 operations from 1 API versions: `2020-07-01-preview`. Use crate feature `package-2020-07-preview` to enable. The operations will be in the `package_2020_07_preview` module.
- `package-2020-01` has 15 operations from 1 API versions: `2020-01-01`. Use crate feature `package-2020-01` to enable. The operations will be in the `package_2020_01` module.
- `package-2017-04` has 15 operations from 1 API versions: `2017-04-19`. Use crate feature `package-2017-04` to enable. The operations will be in the `package_2017_04` module.
- `package-2017-03` has 9 operations from 1 API versions: `2017-03-31`. Use crate feature `package-2017-03` to enable. The operations will be in the `package_2017_03` module.
- `package-2016-07-preview` has 9 operations from 1 API versions: `2016-07-12-preview`. Use crate feature `package-2016-07-preview` to enable. The operations will be in the `package_2016_07_preview` module.