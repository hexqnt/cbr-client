/// Базовый URL API ЦБ РФ.
pub const DEFAULT_BASE_URL: &str = "https://www.cbr.ru/dataservice";

pub(crate) fn normalize_base_url(base_url: impl Into<String>) -> String {
    let mut base_url = base_url.into();
    let without_leading_whitespace = base_url.trim_start();

    if without_leading_whitespace.is_empty() {
        DEFAULT_BASE_URL.to_owned()
    } else {
        let normalized = without_leading_whitespace.trim_end().trim_end_matches('/');

        if without_leading_whitespace.len() != base_url.len() {
            normalized.to_owned()
        } else {
            base_url.truncate(normalized.len());
            base_url
        }
    }
}

pub(crate) fn endpoint(base_url: &str, path: &str) -> String {
    format!("{}/{}", base_url, path.trim_start_matches('/'))
}

macro_rules! configure_reqwest_builder {
    (
        $builder:expr,
        timeout = $timeout:expr,
        use_system_proxy = $use_system_proxy:expr,
        proxy_url = $proxy_url:expr,
        user_agent = $user_agent:expr
    ) => {{
        let mut builder = $builder.timeout($timeout).http1_only();

        if !$use_system_proxy && $proxy_url.is_none() {
            builder = builder.no_proxy();
        }

        if let Some(proxy_url) = $proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).map_err(crate::error::CbrError::build)?;
            builder = builder.proxy(proxy);
        }

        if let Some(user_agent) = $user_agent {
            builder = builder.user_agent(user_agent);
        }

        builder
    }};
}

macro_rules! cbr_endpoint_methods {
    ($impl_macro:ident) => {
        $impl_macro!(
            "Возвращает список публикаций (`/publications`).",
            publications,
            (),
            Vec<Publication>,
            "/publications",
            no_query
        );
        $impl_macro!(
            "Возвращает список показателей публикации (`/datasets`).",
            datasets,
            (publication_id: PublicationId),
            Vec<Dataset>,
            "/datasets",
            query(publication_id_query(publication_id))
        );
        $impl_macro!(
            "Возвращает разрезы показателя (`/measures`).",
            measures,
            (dataset_id: DatasetId),
            MeasuresResponse,
            "/measures",
            query(dataset_id_query(dataset_id))
        );
        $impl_macro!(
            "Возвращает доступный диапазон годов (`/years`).",
            years,
            (dataset_id: DatasetId, measure_id: Option<MeasureId>),
            Vec<YearRange>,
            "/years",
            query(years_query(dataset_id, measure_id))
        );
        $impl_macro!(
            "Возвращает диапазон годов в расширенном формате (`/yearsEx`).",
            years_ex,
            (publication_id: PublicationId, ids: &[DatasetId]),
            Vec<YearRange>,
            "/yearsEx",
            query(years_ex_query(publication_id, ids))
        );
        $impl_macro!(
            "Возвращает показатели и разрезы публикации (`/datasetsEx`).",
            datasets_ex,
            (publication_id: PublicationId),
            DatasetsExResponse,
            "/datasetsEx",
            query(publication_id_query(publication_id))
        );
        $impl_macro!(
            "Возвращает данные для таблицы (`/data`).",
            data,
            (query: DataQuery),
            DataResponse,
            "/data",
            query(query)
        );
        $impl_macro!(
            "Возвращает данные в расширенном формате (`/dataEx`).",
            data_ex,
            (query: DataExQuery),
            DataExResponse,
            "/dataEx",
            query(query)
        );
        $impl_macro!(
            "Возвращает описание (методологию) показателя (`/DatasetDescription`).",
            dataset_description,
            (dataset_id: DatasetId),
            Vec<DatasetDescription>,
            "/DatasetDescription",
            query(dataset_id_query(dataset_id))
        );
        $impl_macro!(
            "Возвращает список категорий и показателей (`/categoryNew`).",
            category_new,
            (),
            CategoryNewResponse,
            "/categoryNew",
            no_query
        );
        $impl_macro!(
            "Возвращает данные показателей (`/dataNew`).",
            data_new,
            (query: DataNewQuery),
            DataNewResponse,
            "/dataNew",
            query(query)
        );
    };
}

pub(crate) use cbr_endpoint_methods;
pub(crate) use configure_reqwest_builder;

#[cfg(test)]
mod tests {
    use super::{DEFAULT_BASE_URL, endpoint, normalize_base_url};

    #[test]
    fn normalize_base_url_uses_default_for_empty_value() {
        assert_eq!(normalize_base_url("  "), DEFAULT_BASE_URL);
    }

    #[test]
    fn normalize_base_url_trims_trailing_slashes() {
        assert_eq!(
            normalize_base_url("https://example.com/api///"),
            "https://example.com/api"
        );
    }

    #[test]
    fn normalize_base_url_preserves_trimming_with_leading_whitespace() {
        assert_eq!(
            normalize_base_url("  https://example.com/api///  "),
            "https://example.com/api"
        );
    }

    #[test]
    fn endpoint_trims_leading_slashes_in_path() {
        assert_eq!(
            endpoint("https://example.com/api", "//datasets"),
            "https://example.com/api/datasets"
        );
    }
}
