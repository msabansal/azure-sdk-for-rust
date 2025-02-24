# azure_mgmt_sql crate

The is an [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust) crate that is generated from the Azure REST API specifications listed in:

https://github.com/Azure/azure-rest-api-specs/blob/main/specification/sql/resource-manager/readme.md

The default `Tag` is `package-composite-v5`.

The following `Tag`s are available:

- `package-composite-v5` has 449 operations from 5 API versions: `2014-04-01`, `2020-11-01-preview`, `2021-02-01-preview`, `2021-05-01-preview`, `2021-08-01-preview`. Use crate feature `package-composite-v5` to enable. The operations will be in the `package_composite_v5` module.
- `package-composite-v4` has 372 operations from 9 API versions: `2014-04-01`, `2015-05-01-preview`, `2017-03-01-preview`, `2017-10-01-preview`, `2018-06-01-preview`, `2019-06-01-preview`, `2020-02-02-preview`, `2020-08-01-preview`, `2020-11-01-preview`. Use crate feature `package-composite-v4` to enable. The operations will be in the `package_composite_v4` module.
- `package-composite-v3` has 364 operations from 8 API versions: `2014-04-01`, `2015-05-01-preview`, `2017-03-01-preview`, `2017-10-01-preview`, `2018-06-01-preview`, `2019-06-01-preview`, `2020-02-02-preview`, `2020-11-01-preview`. Use crate feature `package-composite-v3` to enable. The operations will be in the `package_composite_v3` module.
- `package-composite-v2` has 362 operations from 8 API versions: `2014-04-01`, `2015-05-01-preview`, `2017-03-01-preview`, `2017-10-01-preview`, `2018-06-01-preview`, `2019-06-01-preview`, `2020-02-02-preview`, `2020-11-01-preview`. Use crate feature `package-composite-v2` to enable. The operations will be in the `package_composite_v2` module.
- `package-composite-v1` has 349 operations from 8 API versions: `2014-04-01`, `2015-05-01-preview`, `2017-03-01-preview`, `2017-10-01-preview`, `2018-06-01-preview`, `2019-06-01-preview`, `2020-02-02-preview`, `2020-11-01-preview`. Use crate feature `package-composite-v1` to enable. The operations will be in the `package_composite_v1` module.
- `package-2017-03-preview` has 230 operations from 3 API versions: `2014-04-01`, `2015-05-01-preview`, `2017-03-01-preview`. Use crate feature `package-2017-03-preview` to enable. The operations will be in the `package_2017_03_preview` module.
- `package-2015-05-preview` has 136 operations from 3 API versions: `2014-04-01`, `2015-05-01`, `2015-05-01-preview`. Use crate feature `package-2015-05-preview` to enable. The operations will be in the `package_2015_05_preview` module.
- `package-2014-04` has 42 operations from 1 API versions: `2014-04-01`. Use crate feature `package-2014-04` to enable. The operations will be in the `package_2014_04` module.