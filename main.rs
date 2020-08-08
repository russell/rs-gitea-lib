
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde;

#[macro_use]
extern crate clap;

pub mod cli {
    include!("./cli.rs");
}

pub mod access_token {
    include!("./access_token.rs");
}

pub mod add_collaborator_option {
    include!("./add_collaborator_option.rs");
}

pub mod add_time_option {
    include!("./add_time_option.rs");
}

pub mod annotated_tag {
    include!("./annotated_tag.rs");
}

pub mod annotated_tag_object {
    include!("./annotated_tag_object.rs");
}

pub mod api_error {
    include!("./api_error.rs");
}

pub mod attachment {
    include!("./attachment.rs");
}

pub mod branch {
    include!("./branch.rs");
}

pub mod comment {
    include!("./comment.rs");
}

pub mod commit {
    include!("./commit.rs");
}

pub mod commit_date_options {
    include!("./commit_date_options.rs");
}

pub mod commit_meta {
    include!("./commit_meta.rs");
}

pub mod commit_user {
    include!("./commit_user.rs");
}

pub mod contents_response {
    include!("./contents_response.rs");
}

pub mod create_email_option {
    include!("./create_email_option.rs");
}

pub mod create_file_options {
    include!("./create_file_options.rs");
}

pub mod create_fork_option {
    include!("./create_fork_option.rs");
}

pub mod create_gpg_key_option {
    include!("./create_gpg_key_option.rs");
}

pub mod create_hook_option {
    include!("./create_hook_option.rs");
}

pub mod create_issue_comment_option {
    include!("./create_issue_comment_option.rs");
}

pub mod create_issue_option {
    include!("./create_issue_option.rs");
}

pub mod create_key_option {
    include!("./create_key_option.rs");
}

pub mod create_label_option {
    include!("./create_label_option.rs");
}

pub mod create_milestone_option {
    include!("./create_milestone_option.rs");
}

pub mod create_org_option {
    include!("./create_org_option.rs");
}

pub mod create_pull_request_option {
    include!("./create_pull_request_option.rs");
}

pub mod create_release_option {
    include!("./create_release_option.rs");
}

pub mod create_repo_option {
    include!("./create_repo_option.rs");
}

pub mod create_status_option {
    include!("./create_status_option.rs");
}

pub mod create_team_option {
    include!("./create_team_option.rs");
}

pub mod create_user_option {
    include!("./create_user_option.rs");
}

pub mod delete_email_option {
    include!("./delete_email_option.rs");
}

pub mod delete_file_options {
    include!("./delete_file_options.rs");
}

pub mod deploy_key {
    include!("./deploy_key.rs");
}

pub mod edit_attachment_options {
    include!("./edit_attachment_options.rs");
}

pub mod edit_deadline_option {
    include!("./edit_deadline_option.rs");
}

pub mod edit_git_hook_option {
    include!("./edit_git_hook_option.rs");
}

pub mod edit_hook_option {
    include!("./edit_hook_option.rs");
}

pub mod edit_issue_comment_option {
    include!("./edit_issue_comment_option.rs");
}

pub mod edit_issue_option {
    include!("./edit_issue_option.rs");
}

pub mod edit_label_option {
    include!("./edit_label_option.rs");
}

pub mod edit_milestone_option {
    include!("./edit_milestone_option.rs");
}

pub mod edit_org_option {
    include!("./edit_org_option.rs");
}

pub mod edit_pull_request_option {
    include!("./edit_pull_request_option.rs");
}

pub mod edit_reaction_option {
    include!("./edit_reaction_option.rs");
}

pub mod edit_release_option {
    include!("./edit_release_option.rs");
}

pub mod edit_repo_option {
    include!("./edit_repo_option.rs");
}

pub mod edit_team_option {
    include!("./edit_team_option.rs");
}

pub mod edit_user_option {
    include!("./edit_user_option.rs");
}

pub mod email {
    include!("./email.rs");
}

pub mod external_tracker {
    include!("./external_tracker.rs");
}

pub mod external_wiki {
    include!("./external_wiki.rs");
}

pub mod file_commit_response {
    include!("./file_commit_response.rs");
}

pub mod file_delete_response {
    include!("./file_delete_response.rs");
}

pub mod file_links_response {
    include!("./file_links_response.rs");
}

pub mod file_response {
    include!("./file_response.rs");
}

pub mod get_orgs_org_teams_search_response {
    include!("./get_orgs_org_teams_search_response.rs");
}

pub mod get_users_search_response {
    include!("./get_users_search_response.rs");
}

pub mod git_blob_response {
    include!("./git_blob_response.rs");
}

pub mod git_entry {
    include!("./git_entry.rs");
}

pub mod git_hook {
    include!("./git_hook.rs");
}

pub mod git_object {
    include!("./git_object.rs");
}

pub mod git_tree_response {
    include!("./git_tree_response.rs");
}

pub mod gpg_key {
    include!("./gpg_key.rs");
}

pub mod gpg_key_email {
    include!("./gpg_key_email.rs");
}

pub mod hook {
    include!("./hook.rs");
}

pub mod identity {
    include!("./identity.rs");
}

pub mod internal_tracker {
    include!("./internal_tracker.rs");
}

pub mod issue {
    include!("./issue.rs");
}

pub mod issue_deadline {
    include!("./issue_deadline.rs");
}

pub mod issue_labels_option {
    include!("./issue_labels_option.rs");
}

pub mod label {
    include!("./label.rs");
}

pub mod markdown_option {
    include!("./markdown_option.rs");
}

pub mod merge_pull_request_option {
    include!("./merge_pull_request_option.rs");
}

pub mod migrate_repo_form {
    include!("./migrate_repo_form.rs");
}

pub mod milestone {
    include!("./milestone.rs");
}

pub mod miscellaneous {
    include!("./miscellaneous.rs");
}

pub mod organization {
    include!("./organization.rs");
}

pub mod payload_commit {
    include!("./payload_commit.rs");
}

pub mod payload_commit_verification {
    include!("./payload_commit_verification.rs");
}

pub mod payload_user {
    include!("./payload_user.rs");
}

pub mod permission {
    include!("./permission.rs");
}

pub mod post_users_username_tokens_body {
    include!("./post_users_username_tokens_body.rs");
}

pub mod pr_branch_info {
    include!("./pr_branch_info.rs");
}

pub mod public_key {
    include!("./public_key.rs");
}

pub mod pull_request {
    include!("./pull_request.rs");
}

pub mod pull_request_meta {
    include!("./pull_request_meta.rs");
}

pub mod reaction_response {
    include!("./reaction_response.rs");
}

pub mod reference {
    include!("./reference.rs");
}

pub mod release {
    include!("./release.rs");
}

pub mod repo_commit {
    include!("./repo_commit.rs");
}

pub mod repo_topic_options {
    include!("./repo_topic_options.rs");
}

pub mod repository {
    include!("./repository.rs");
}

pub mod repository_meta {
    include!("./repository_meta.rs");
}

pub mod search_results {
    include!("./search_results.rs");
}

pub mod server_version {
    include!("./server_version.rs");
}

pub mod status {
    include!("./status.rs");
}

pub mod stop_watch {
    include!("./stop_watch.rs");
}

pub mod tag {
    include!("./tag.rs");
}

pub mod team {
    include!("./team.rs");
}

pub mod topic_name {
    include!("./topic_name.rs");
}

pub mod topic_response {
    include!("./topic_response.rs");
}

pub mod tracked_time {
    include!("./tracked_time.rs");
}

pub mod update_file_options {
    include!("./update_file_options.rs");
}

pub mod user {
    include!("./user.rs");
}

pub mod user_heatmap_data {
    include!("./user_heatmap_data.rs");
}

pub mod watch_info {
    include!("./watch_info.rs");
}

pub mod client {
    use failure::Fail;
    use futures::Stream;
    use parking_lot::Mutex;

    use std::borrow::Cow;
    use std::fmt::Debug;
    use std::path::Path;

    /// Common API errors.
    #[derive(Debug, Fail)]
    pub enum ApiError<R: Debug + Send + 'static> {
        #[fail(display = "API request failed for path: {} (code: {})", _0, _1)]
        Failure(String, http::status::StatusCode, Mutex<R>),
        #[fail(display = "Unsupported media type in response: {}", _0)]
        UnsupportedMediaType(String, Mutex<R>),
        #[fail(display = "An error has occurred while performing the API request: {}", _0)]
        Reqwest(reqwest::Error),
        #[fail(display = "I/O error: {}", _0)]
        Io(std::io::Error),
        #[fail(display = "Error en/decoding \"application/json\" data: {}", _0)]
        ApplicationJson(serde_json::Error),
        #[fail(display = "Error en/decoding \"application/yaml\" data: {}", _0)]
        ApplicationYaml(serde_yaml::Error),
    }

    /// Form object for building multipart request body.
    pub trait Form: Sized {
        /// Creates a new builder.
        fn new() -> Self;

        /// Adds the given key and value as text.
        fn text<T, U>(self, key: T, value: U) -> Self
            where T: Into<Cow<'static, str>>,
                  U: Into<Cow<'static, str>>;

        /// Adds the file from the given path for streaming.
        fn file<K>(self, key: K, path: &Path) -> std::io::Result<Self>
            where K: Into<Cow<'static, str>>;
    }

    /// HTTP Request.
    pub trait Request {
        type Form: Form;

        /// Sets the header with the given key and value.
        fn header(self, name: &'static str, value: &str) -> Self;

        /// Sets body using the given vector of bytes.
        ///
        /// **NOTE:** Appropriate `Content-Type` header must be set
        /// after calling this method.
        fn body_bytes(self, body: Vec<u8>) -> Self;

        /// Sets JSON body based on the given value.
        fn json<T: serde::Serialize>(self, value: &T) -> Self;

        /// Sets `multipart/form-data` body using the given form.
        fn multipart_form_data(self, form: Self::Form) -> Self;

        /// Sets/adds query parameters based on the given value.
        ///
        /// **NOTE:** This method must be called only once. It's unspecified
        /// as to whether this appends/replaces query parameters.
        fn query<T: serde::Serialize>(self, params: &T) -> Self;
    }

    impl Form for reqwest::multipart::Form {
        fn new() -> Self {
            reqwest::multipart::Form::new()
        }

        fn text<T, U>(self, key: T, value: U) -> Self
            where T: Into<Cow<'static, str>>,
                  U: Into<Cow<'static, str>>
        {
            reqwest::multipart::Form::text(self, key, value)
        }

        fn file<K>(self, key: K, path: &Path) -> std::io::Result<Self>
            where K: Into<Cow<'static, str>>
        {
            use reqwest::multipart::{Form, Part};
            use tokio_util::codec::{BytesCodec, FramedRead};

            let fd = std::fs::File::open(path)?;
            let reader = tokio::fs::File::from_std(fd);
            let bytes_stream = FramedRead::new(reader, BytesCodec::new());
            let part = Part::stream(reqwest::Body::wrap_stream(bytes_stream));
            Ok(Form::part(self, key, part))
        }
    }

    impl Request for reqwest::RequestBuilder {
        type Form = reqwest::multipart::Form;

        fn header(self, name: &'static str, value: &str) -> Self {
            reqwest::RequestBuilder::header(self, name, value)
        }

        fn multipart_form_data(self, form: Self::Form) -> Self {
            self.multipart(form)
        }

        fn body_bytes(self, body: Vec<u8>) -> Self {
            self.body(body)
        }

        fn json<T: serde::Serialize>(self, value: &T) -> Self {
            <reqwest::RequestBuilder>::json(self, value)
        }

        fn query<T: serde::Serialize>(self, params: &T) -> Self {
            reqwest::RequestBuilder::query(self, params)
        }
    }

    /// HTTP Response.
    #[async_trait::async_trait]
    pub trait Response: Debug + Send + Sized {
        type Bytes: AsRef<[u8]>;
        type Error;

        /// Gets the value for the given header name, if any.
        fn header(&self, name: &'static str) -> Option<&str>;

        /// Takes all headers from the response.
        fn take_headers(&mut self) -> http::header::HeaderMap;

        /// Status code for this response.
        fn status(&self) -> http::status::StatusCode;

        /// Media type for this response body (if any).
        fn media_type(&self) -> Option<mime::MediaType>;

        /// Response body as a stream.
        fn stream(self) -> Box<dyn Stream<Item=Result<Self::Bytes, Self::Error>> + Unpin>;

        /// Vector of bytes from the response body.
        async fn body_bytes(self) -> Result<Self::Bytes, ApiError<Self>>;
    }

    #[async_trait::async_trait]
    impl Response for reqwest::Response {
        type Bytes = bytes::Bytes;
        type Error = reqwest::Error;

        fn header(&self, name: &'static str) -> Option<&str> {
            self.headers().get(name).and_then(|v| v.to_str().ok())
        }

        fn take_headers(&mut self) -> http::header::HeaderMap {
            std::mem::replace(self.headers_mut(), http::header::HeaderMap::new())
        }

        fn status(&self) -> http::status::StatusCode {
            reqwest::Response::status(self)
        }

        fn media_type(&self) -> Option<mime::MediaType> {
            self.header(http::header::CONTENT_TYPE.as_str())
                .and_then(|v| v.parse().ok())
        }

        fn stream(self) -> Box<dyn Stream<Item=Result<Self::Bytes, Self::Error>> + Unpin> {
            Box::new(self.bytes_stream()) as Box<_>
        }

        async fn body_bytes(self) -> Result<Self::Bytes, ApiError<Self>> {
            Ok(self.bytes().await.map_err(ApiError::Reqwest)?)
        }
    }

    /// Represents an API client.
    #[async_trait::async_trait]
    pub trait ApiClient {
        type Request: Request + Send;
        type Response: Response;

        /// Consumes a method and a relative path and produces a request builder for a single API call.
        fn request_builder(&self, method: http::Method, rel_path: &str) -> Self::Request;

        /// Performs the HTTP request using the given `Request` object
        /// and returns a `Response` future.
        async fn make_request(&self, req: Self::Request) -> Result<Self::Response, ApiError<Self::Response>>;
    }

    #[async_trait::async_trait]
    impl ApiClient for reqwest::Client {
        type Request = reqwest::RequestBuilder;
        type Response = reqwest::Response;

        fn request_builder(&self, method: http::Method, rel_path: &str) -> Self::Request {
            let mut u = String::from("https://example.com/api/v1");
            u.push_str(rel_path.trim_start_matches('/'));
            self.request(method, &u)
        }

        async fn make_request(&self, req: Self::Request) -> Result<Self::Response, ApiError<Self::Response>> {
            let req = req.build().map_err(ApiError::Reqwest)?;
            let resp = self.execute(req).await.map_err(ApiError::Reqwest)?;
            Ok(resp)
        }
    }

    /// A trait for indicating that the implementor can send an API call.
    #[async_trait::async_trait]
    pub trait Sendable<Client>
    where
        Client: ApiClient + Sync + 'static,
        Self: Sized
    {
        /// The output object from this API request.
        type Output: serde::de::DeserializeOwned;

        /// HTTP method used by this call.
        const METHOD: http::Method;

        /// Relative URL for this API call formatted appropriately with parameter values.
        ///
        /// **NOTE:** This URL **must** begin with `/`.
        fn rel_path(&self) -> std::borrow::Cow<'static, str>;

        /// Modifier for this object. Builders override this method if they
        /// wish to add query parameters, set body, etc.
        fn modify(&self, req: Client::Request) -> Result<Client::Request, ApiError<Client::Response>> {
            Ok(req)
        }

        /// Sends the request and returns a future for the response object.
        async fn send(&self, client: &Client) -> Result<ResponseWrapper<Self::Output, Self>, ApiError<Client::Response>> {
            let resp = self.send_raw(client).await?;
            let media = resp.media_type();
            if let Some(ty) = media {
                if media_types::M_0.matches(&ty) {
                    return ResponseWrapper::wrap(resp, |r| async {
                        let bytes = r.body_bytes().await?;
                        serde_json::from_reader(bytes.as_ref()).map_err(ApiError::from)
                    }).await
                }
                else if media_types::M_1.matches(&ty) {
                    return ResponseWrapper::wrap(resp, |r| async {
                        let bytes = r.body_bytes().await?;
                        serde_yaml::from_reader(bytes.as_ref()).map_err(ApiError::from)
                    }).await
                }
            }

            let ty = resp.header(http::header::CONTENT_TYPE.as_str())
                .map(|v| String::from_utf8_lossy(v.as_bytes()).into_owned())
                .unwrap_or_default();
            Err(ApiError::UnsupportedMediaType(ty, Mutex::new(resp)))
        }

        /// Convenience method for returning a raw response after sending a request.
        async fn send_raw(&self, client: &Client) -> Result<Client::Response, ApiError<Client::Response>> {
            let rel_path = self.rel_path();
            let req = self.modify(client.request_builder(Self::METHOD, &rel_path))?;
            let resp = client.make_request(req).await?;
            if resp.status().is_success() {
                Ok(resp)
            } else {
                Err(ApiError::Failure(rel_path.into_owned(), resp.status(), Mutex::new(resp)))
            }
        }
    }

    /// Wrapper containing response-related information.
    pub struct ResponseWrapper<T, B> {
        /// Response object
        pub object: T,
        /// Response headers
        pub headers: http::HeaderMap,
        /// Response status code
        pub status: http::status::StatusCode,
        _builder: core::marker::PhantomData<B>,
    }

    impl<T, B> ResponseWrapper<T, B> {
        pub(crate) async fn wrap<F, R>(mut resp: R, f: impl FnOnce(R) -> F) -> Result<Self, ApiError<R>>
            where F: std::future::Future<Output=Result<T, ApiError<R>>>,
                  R: Response + 'static
        {
            let status = resp.status();
            let headers = resp.take_headers();
            Ok(ResponseWrapper {
                object: f(resp).await?,
                headers,
                status,
                _builder: core::marker::PhantomData,
            })
        }
    }

    impl<'de, T, B> serde::de::Deserialize<'de> for ResponseWrapper<T, B> {
        fn deserialize<D>(_: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>
        {
            unimplemented!("ResponseWrapper is not supposed to be deserialized.");
        }
    }

    impl<T, B> std::ops::Deref for ResponseWrapper<T, B> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.object
        }
    }

    impl<T, B> std::ops::DerefMut for ResponseWrapper<T, B> {
        fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
            &mut self.object
        }
    }

    pub mod media_types {
        use lazy_static::lazy_static;

        lazy_static! {
            pub static ref M_0: mime::MediaRange =
                mime::MediaRange::parse("application/json").expect("cannot parse \"application/json\" as media range");
            pub static ref M_1: mime::MediaRange =
                mime::MediaRange::parse("application/yaml").expect("cannot parse \"application/yaml\" as media range");
        }
    }

    impl<R: Response + 'static> From<std::io::Error> for ApiError<R> {
        fn from(e: std::io::Error) -> Self {
            ApiError::Io(e)
        }
    }

    impl<R: Response + 'static> From<serde_json::Error> for ApiError<R> {
        fn from(e: serde_json::Error) -> Self {
            ApiError::ApplicationJson(e)
        }
    }

    impl<R: Response + 'static> From<serde_yaml::Error> for ApiError<R> {
        fn from(e: serde_yaml::Error) -> Self {
            ApiError::ApplicationYaml(e)
        }
    }
}

pub mod generics {
    include!("./generics.rs");
}

pub mod util {
    include!("./util.rs");
}

use self::client::{ApiClient, ApiError, Response};
use self::util::ResponseStream;
use clap::{App, ArgMatches};
use failure::Error;
use openssl::pkcs12::Pkcs12;
use openssl::pkey::PKey;
use openssl::x509::X509;

use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Fail)]
#[allow(dead_code)]
enum ClientError {
    #[fail(display = "Duration parse error: {}", _0)]
    Duration(humantime::DurationError),
    #[fail(display = "I/O error: {}", _0)]
    Io(std::io::Error),
    #[fail(display = "OpenSSL error: {}", _0)]
    OpenSsl(openssl::error::ErrorStack),
    #[fail(display = "Client error: {}", _0)]
    Reqwest(reqwest::Error),
    #[fail(display = "URL error: {}", _0)]
    Url(url::ParseError),
    #[fail(display = "{}", _0)]
    Api(self::client::ApiError<reqwest::Response>),
    #[fail(display = "")]
    Empty,
}

impl From<ApiError<reqwest::Response>> for ClientError {
    fn from(e: ApiError<reqwest::Response>) -> Self {
        ClientError::Api(e)
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let mut data = vec![];
    let mut fd = File::open(path.as_ref()).map_err(ClientError::Io)?;
    fd.read_to_end(&mut data).map_err(ClientError::Io)?;
    Ok(data)
}

#[derive(Clone)]
struct WrappedClient {
    verbose: bool,
    inner: reqwest::Client,
    url: reqwest::Url,
}

#[async_trait::async_trait]
impl ApiClient for WrappedClient {
    type Request = reqwest::RequestBuilder;
    type Response = reqwest::Response;

    async fn make_request(&self, req: Self::Request) -> Result<Self::Response, ApiError<Self::Response>> {
        let req = req.build().map_err(ApiError::Reqwest)?;
        if self.verbose {
            println!("{} {}", req.method(), req.url());
        }

        Ok(self.inner.execute(req).await.map_err(ApiError::Reqwest)?)
    }

    fn request_builder(&self, method: http::Method, rel_path: &str) -> Self::Request {
        let mut u = self.url.clone();
        let mut path = u.path().trim_matches('/').to_owned();
        if !path.is_empty() {
            path = String::from("/") + &path;
        }

        path.push_str(rel_path);
        u.set_path(&path);
        self.inner.request(method, u)
    }
}

fn make_client<'a>(matches: &'a ArgMatches<'a>) -> Result<WrappedClient, Error> {
    let mut client = reqwest::Client::builder();

    if let Some(p) = matches.value_of("ca-cert") {
        let ca_cert = X509::from_pem(&read_file(p)?)
            .map_err(ClientError::OpenSsl)?;
        let ca_der = ca_cert.to_der().map_err(ClientError::OpenSsl)?;
        client = client.add_root_certificate(
            reqwest::Certificate::from_der(&ca_der)
                .map_err(ClientError::Reqwest)?
        );
    }

    // FIXME: Is this the only way?
    if let (Some(p1), Some(p2)) = (matches.value_of("client-key"), matches.value_of("client-cert")) {
        let cert = X509::from_pem(&read_file(p2)?).map_err(ClientError::OpenSsl)?;
        let key = PKey::private_key_from_pem(&read_file(p1)?)
            .map_err(ClientError::OpenSsl)?;
        let builder = Pkcs12::builder();
        let pkcs12 = builder.build("foobar", "my-client", &key, &cert)
            .map_err(ClientError::OpenSsl)?;
        let identity = reqwest::Identity::from_pkcs12_der(
            &pkcs12.to_der().map_err(ClientError::OpenSsl)?,
            "foobar"
        ).map_err(ClientError::Reqwest)?;
        client = client.identity(identity);
    }

    if let Some(timeout) = matches.value_of("timeout") {
        let d = timeout.parse::<humantime::Duration>()?;
        client = client.timeout(d.into());
    }

    let is_verbose = matches.is_present("verbose");
    let url = matches.value_of("url").expect("required arg URL?");
    Ok(WrappedClient {
        inner: client.build().map_err(ClientError::Reqwest)?,
        url: reqwest::Url::parse(url).map_err(ClientError::Url)?,
        verbose: is_verbose,
    })
}

async fn run_app() -> Result<(), Error> {
    let yml = load_yaml!("app.yaml");
    let app = App::from_yaml(yml);
    let matches = app.get_matches();
    let (sub_cmd, sub_matches) = matches.subcommand();

    let client = make_client(&matches)?;
    let response = self::cli::fetch_response(&client, &matches, sub_cmd, sub_matches).await?;

    let status = response.status();
    if client.verbose {
        println!("{}", status);
    }

    let mut stdout = tokio::io::stdout();
    ResponseStream(response.stream()).to_writer(&mut stdout).await?;
    if !status.is_success() {
        Err(ClientError::Empty)?
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    if let Err(e) = run_app().await {
        println!("{}", e);
    }
}
