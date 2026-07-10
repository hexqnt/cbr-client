use std::time::Duration;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::client_common::{
    cbr_endpoint_methods, configure_reqwest_builder, endpoint, normalize_base_url,
};
use crate::error::{CbrError, parse_json_body};
use crate::models::{
    CategoryNewResponse, DataExResponse, DataNewResponse, DataResponse, Dataset,
    DatasetDescription, DatasetsExResponse, MeasuresResponse, Publication, YearRange,
};
use crate::query::{
    DataExQuery, DataNewQuery, DataQuery, dataset_id_query, publication_id_query, years_ex_query,
    years_query,
};
use crate::types::{DatasetId, MeasureId, PublicationId};

pub use crate::client_common::DEFAULT_BASE_URL;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

macro_rules! impl_async_endpoint_method {
    (
        $doc:literal,
        $name:ident,
        ($($arg_name:ident : $arg_ty:ty),* $(,)?),
        $ret:ty,
        $path:literal,
        no_query
    ) => {
        #[doc = $doc]
        #[inline]
        pub async fn $name(&self $(, $arg_name: $arg_ty)*) -> Result<$ret, CbrError> {
            self.request_json($path).await
        }
    };
    (
        $doc:literal,
        $name:ident,
        ($($arg_name:ident : $arg_ty:ty),* $(,)?),
        $ret:ty,
        $path:literal,
        query($query:expr)
    ) => {
        #[doc = $doc]
        #[inline]
        pub async fn $name(&self $(, $arg_name: $arg_ty)*) -> Result<$ret, CbrError> {
            self.request_json_with_query($path, &$query).await
        }
    };
}

/// Builder асинхронного клиента [`CbrClient`].
#[derive(Debug, Clone)]
pub struct CbrClientBuilder {
    pub(crate) base_url: String,
    pub(crate) timeout: Duration,
    pub(crate) user_agent: Option<String>,
    pub(crate) proxy_url: Option<String>,
    pub(crate) use_system_proxy: bool,
}

impl CbrClientBuilder {
    /// Создаёт builder с настройками по умолчанию.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Устанавливает базовый URL API.
    ///
    /// Если передана пустая строка, будет использован [`DEFAULT_BASE_URL`].
    #[must_use]
    #[inline]
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = normalize_base_url(base_url);
        self
    }

    /// Устанавливает timeout для HTTP-запросов.
    #[must_use]
    #[inline]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Устанавливает заголовок `User-Agent`.
    #[must_use]
    #[inline]
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Устанавливает явный proxy URL для всех HTTP/HTTPS-запросов.
    ///
    /// Примеры:
    /// - `http://127.0.0.1:8080`
    /// - `socks5h://127.0.0.1:1080`
    #[must_use]
    #[inline]
    pub fn proxy(mut self, proxy_url: impl Into<String>) -> Self {
        self.proxy_url = Some(proxy_url.into());
        self
    }

    /// Включает или отключает использование системных proxy-настроек
    /// (`HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY`, `NO_PROXY`).
    ///
    /// По умолчанию отключено (`false`) для предсказуемого поведения.
    #[must_use]
    #[inline]
    pub fn use_system_proxy(mut self, enabled: bool) -> Self {
        self.use_system_proxy = enabled;
        self
    }

    /// Собирает асинхронный клиент.
    pub fn build(self) -> Result<CbrClient, CbrError> {
        // Принудительно используем HTTP/1.1 для стабильной работы с API ЦБ и mock-серверами.
        let builder = configure_reqwest_builder!(
            reqwest::Client::builder(),
            timeout = self.timeout,
            use_system_proxy = self.use_system_proxy,
            proxy_url = self.proxy_url.as_deref(),
            user_agent = self.user_agent.as_deref()
        );

        let http = builder.build().map_err(CbrError::build)?;
        Ok(CbrClient {
            base_url: self.base_url,
            http,
        })
    }

    /// Собирает блокирующий клиент.
    ///
    /// Доступно только с feature `blocking`.
    #[cfg(feature = "blocking")]
    #[inline]
    pub fn build_blocking(self) -> Result<crate::blocking::BlockingCbrClient, CbrError> {
        crate::blocking::BlockingCbrClient::from_builder(self)
    }
}

impl Default for CbrClientBuilder {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_owned(),
            timeout: DEFAULT_TIMEOUT,
            user_agent: None,
            proxy_url: None,
            use_system_proxy: false,
        }
    }
}

/// Асинхронный клиент API ЦБ РФ.
#[derive(Debug, Clone)]
pub struct CbrClient {
    base_url: String,
    http: reqwest::Client,
}

impl CbrClient {
    /// Создаёт клиент с настройками по умолчанию.
    #[inline]
    pub fn new() -> Result<Self, CbrError> {
        Self::builder().build()
    }

    /// Возвращает builder для тонкой настройки клиента.
    #[must_use]
    #[inline]
    pub fn builder() -> CbrClientBuilder {
        CbrClientBuilder::new()
    }

    /// Возвращает текущий базовый URL клиента.
    #[must_use]
    #[inline]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Выполняет GET-запрос к произвольному endpoint и десериализует JSON в тип пользователя.
    ///
    /// `path` указывается относительно `base_url`. Начальный `/` опционален.
    #[inline]
    pub async fn request_json<T>(&self, path: &str) -> Result<T, CbrError>
    where
        T: DeserializeOwned,
    {
        self.get_json(path).await
    }

    /// Выполняет GET-запрос с query-параметрами и десериализует JSON в тип пользователя.
    ///
    /// `path` указывается относительно `base_url`. Начальный `/` опционален.
    #[inline]
    pub async fn request_json_with_query<T, Q>(&self, path: &str, query: &Q) -> Result<T, CbrError>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        self.get_json_with_query(path, query).await
    }

    cbr_endpoint_methods!(impl_async_endpoint_method);

    async fn get_json<T>(&self, path: &str) -> Result<T, CbrError>
    where
        T: DeserializeOwned,
    {
        let response = self
            .http
            .get(endpoint(&self.base_url, path))
            .send()
            .await
            .map_err(CbrError::transport)?;
        let status = response.status();
        let body = response.bytes().await.map_err(CbrError::transport)?;
        parse_json_body(status, body.as_ref())
    }

    async fn get_json_with_query<T, Q>(&self, path: &str, query: &Q) -> Result<T, CbrError>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let response = self
            .http
            .get(endpoint(&self.base_url, path))
            .query(query)
            .send()
            .await
            .map_err(CbrError::transport)?;
        let status = response.status();
        let body = response.bytes().await.map_err(CbrError::transport)?;
        parse_json_body(status, body.as_ref())
    }
}
