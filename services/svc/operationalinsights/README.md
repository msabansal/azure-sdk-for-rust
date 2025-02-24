# azure_svc_operationalinsights crate

The is an [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust) crate that is generated from the Azure REST API specifications listed in:

https://github.com/Azure/azure-rest-api-specs/blob/main/specification/operationalinsights/data-plane/readme.md

The default `Tag` is `v1`.

The following `Tag`s are available:

- `v1` has 4 operations from 1 API versions: `v1`. Use crate feature `v1` to enable. The operations will be in the `v1` module.
- `20171001` has 4 operations from 1 API versions: `2017-10-01`. Use crate feature `20171001` to enable. The operations will be in the `v20171001` module.
- `20210519` has 5 operations from 1 API versions: `2021-05-19_Preview`. Use crate feature `20210519` to enable. The operations will be in the `v20210519` module.