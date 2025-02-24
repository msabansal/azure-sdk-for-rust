#![doc = "generated by AutoRust"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = "https://sts.mixedreality.azure.com";
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: impl Into<azure_core::Request>) -> azure_core::error::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
}
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    GetToken(#[from] get_token::Error),
}
impl Client {
    #[doc = "Gets an access token to be used with Mixed Reality services."]
    pub fn get_token(&self, account_id: impl Into<String>) -> get_token::Builder {
        get_token::Builder {
            client: self.clone(),
            account_id: account_id.into(),
            x_mrc_cv: None,
        }
    }
}
pub mod get_token {
    use super::models;
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("Error response #response_type")]
        BadRequest400 {},
        #[error("Error response #response_type")]
        Unauthorized401 {},
        #[error("Error response #response_type")]
        TooManyRequests429 {},
        #[error("HTTP status code {}", status_code)]
        DefaultResponse { status_code: http::StatusCode },
        #[error("Failed to parse request URL")]
        ParseUrl(#[source] url::ParseError),
        #[error("Failed to build request")]
        BuildRequest(#[source] http::Error),
        #[error("Failed to serialize request body")]
        Serialize(#[source] serde_json::Error),
        #[error("Failed to get access token")]
        GetToken(#[source] azure_core::Error),
        #[error("Failed to execute request")]
        SendRequest(#[source] azure_core::error::Error),
        #[error("Failed to get response bytes")]
        ResponseBytes(#[source] azure_core::error::Error),
        #[error("Failed to deserialize response, body: {1:?}")]
        Deserialize(#[source] serde_json::Error, bytes::Bytes),
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) account_id: String,
        pub(crate) x_mrc_cv: Option<String>,
    }
    impl Builder {
        pub fn x_mrc_cv(mut self, x_mrc_cv: impl Into<String>) -> Self {
            self.x_mrc_cv = Some(x_mrc_cv.into());
            self
        }
        pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::StsTokenResponseMessage, Error>> {
            Box::pin(async move {
                let url_str = &format!("{}/Accounts/{}/token", self.client.endpoint(), &self.account_id);
                let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                let mut req_builder = http::request::Builder::new();
                req_builder = req_builder.method(http::Method::GET);
                let credential = self.client.token_credential();
                let token_response = credential
                    .get_token(&self.client.scopes().join(" "))
                    .await
                    .map_err(Error::GetToken)?;
                req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                url.query_pairs_mut().append_pair("api-version", "2019-02-28-preview");
                if let Some(x_mrc_cv) = &self.x_mrc_cv {
                    req_builder = req_builder.header("X-MRC-CV", x_mrc_cv);
                }
                let req_body = azure_core::EMPTY_BODY;
                req_builder = req_builder.uri(url.as_str());
                let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                match rsp_status {
                    http::StatusCode::OK => {
                        let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                        let rsp_value: models::StsTokenResponseMessage =
                            serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                        Ok(rsp_value)
                    }
                    http::StatusCode::BAD_REQUEST => Err(Error::BadRequest400 {}),
                    http::StatusCode::UNAUTHORIZED => Err(Error::Unauthorized401 {}),
                    http::StatusCode::TOO_MANY_REQUESTS => Err(Error::TooManyRequests429 {}),
                    status_code => Err(Error::DefaultResponse { status_code }),
                }
            })
        }
    }
}
