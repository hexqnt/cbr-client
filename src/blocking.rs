#![cfg(feature = "blocking")]

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::client::CbrClientBuilder;
use crate::client_common::{cbr_endpoint_methods, configure_reqwest_builder, endpoint};
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

macro_rules! impl_blocking_endpoint_method {
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
        pub fn $name(&self $(, $arg_name: $arg_ty)*) -> Result<$ret, CbrError> {
            self.request_json($path)
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
        pub fn $name(&self $(, $arg_name: $arg_ty)*) -> Result<$ret, CbrError> {
            self.request_json_with_query($path, &$query)
        }
    };
}

/// Блокирующий клиент API ЦБ РФ.
#[derive(Debug, Clone)]
pub struct BlockingCbrClient {
    base_url: String,
    http: reqwest::blocking::Client,
}

impl BlockingCbrClient {
    /// Создаёт клиент с настройками по умолчанию.
    #[inline]
    pub fn new() -> Result<Self, CbrError> {
        Self::builder().build_blocking()
    }

    /// Возвращает builder для настройки клиента.
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
    pub fn request_json<T>(&self, path: &str) -> Result<T, CbrError>
    where
        T: DeserializeOwned,
    {
        self.get_json(path)
    }

    /// Выполняет GET-запрос с query-параметрами и десериализует JSON в тип пользователя.
    ///
    /// `path` указывается относительно `base_url`. Начальный `/` опционален.
    #[inline]
    pub fn request_json_with_query<T, Q>(&self, path: &str, query: &Q) -> Result<T, CbrError>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        self.get_json_with_query(path, query)
    }

    cbr_endpoint_methods!(impl_blocking_endpoint_method);

    pub(crate) fn from_builder(builder: CbrClientBuilder) -> Result<Self, CbrError> {
        // Настройки идентичны async-клиенту для предсказуемого поведения.
        let client_builder = configure_reqwest_builder!(
            reqwest::blocking::Client::builder(),
            timeout = builder.timeout,
            use_system_proxy = builder.use_system_proxy,
            proxy_url = builder.proxy_url.as_deref(),
            user_agent = builder.user_agent.as_deref()
        );

        let http = client_builder.build().map_err(CbrError::build)?;
        Ok(Self {
            base_url: builder.base_url,
            http,
        })
    }

    fn get_json<T>(&self, path: &str) -> Result<T, CbrError>
    where
        T: DeserializeOwned,
    {
        let response = self
            .http
            .get(endpoint(&self.base_url, path))
            .send()
            .map_err(CbrError::transport)?;
        let status = response.status();
        let body = response.bytes().map_err(CbrError::transport)?;
        parse_json_body(status, body.as_ref())
    }

    fn get_json_with_query<T, Q>(&self, path: &str, query: &Q) -> Result<T, CbrError>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let response = self
            .http
            .get(endpoint(&self.base_url, path))
            .query(query)
            .send()
            .map_err(CbrError::transport)?;
        let status = response.status();
        let body = response.bytes().map_err(CbrError::transport)?;
        parse_json_body(status, body.as_ref())
    }
}
