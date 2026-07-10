use cbr_client::presets::fx::{self, FxMetric, FxPeriodicity};
use cbr_client::{
    CategoryId, CbrClient, CbrError, DataExQuery, DataNewQuery, DataQuery, DatasetId, IndicatorId,
    MeasureId, PublicationId, SortKey, Year, YearSpan,
};
use httpmock::prelude::*;
use serde::{Deserialize, Serialize};

const JSON_CONTENT_TYPE: &str = "application/json";

const PUBLICATIONS_JSON: &str = r#"[{"id":1,"parent_id":-1,"category_name":"Rates","NoActive":1}]"#;
const DATASETS_JSON: &str = r#"[{"id":37,"parent_id":-1,"name":"Name","full_name":"Full","type":1,"reporting":"period","link":"/l","updated_time":"2026-02-09T00:00:00"}]"#;
const MEASURES_JSON: &str = r#"{"measure":[{"id":2,"parent_id":1,"name":"RUB","sort":2}]}"#;
const YEARS_JSON: &str = r#"[{"FromYear":2014,"ToYear":2025}]"#;
const DATASETS_EX_JSON: &str = r#"{"indicators":[{"id":37,"parent_id":-1,"name":"Name"}],"measures_1":[{"id":2,"parent_id":1,"name":"RUB"}],"measures_2":[{"id":9,"parent_id":-1,"name":"1y-3y"}],"units":[{"id":1,"parent_id":-1,"name":"%"}],"years":[{"FromYear":2014,"ToYear":2025}]}"#;
const DATA_JSON: &str = r#"{"RawData":[{"colId":2,"element_id":2,"measure_id":2,"unit_id":1,"obs_val":3.31,"rowId":349,"dt":"Jan 2021","periodicity":"month","date":"2021-02-01T00:00:00","digits":2}],"headerData":[{"id":2,"elname":"Up to 30 days"}],"units":[{"id":1,"val":"%"}],"DTRange":[{"FromY":2014,"ToY":2025}],"SType":[{"sType":1,"dsName":"Name","PublName":"Publ"}]}"#;
const DATA_EX_JSON: &str = r#"{"RawData":[{"indicator_id":38,"measure_1_id":2,"measure_2_id":9,"unit_id":1,"value":4.6,"period_id":349,"period":"Jan 2021","periodicity":"month","date":"01.02.2021","rowId":2710568}],"links":[{"indicator_id":38,"measure_1_id":2,"measure_2_id":9,"unit_id":1,"name":"Name","sSort":"38"}]}"#;
const DATASET_DESCRIPTION_JSON: &str = r#"[{"description":"<p>Text</p>"}]"#;
const CATEGORY_NEW_JSON: &str = r#"{"category":[{"category_id":5,"category_name":"Money","indicator_id":7,"indicator_parent":-1,"indicator_name":"M2","link":"/x","begin_dt":1992,"end_dt":2026}]}"#;
const CATEGORY_NEW_FX_JSON: &str = r#"{"category":[{"category_id":33,"category_name":"Статистика внешнего сектора-Основные производные показатели динамики обменного курса рубля-Номинальные курсы иностранных валют к рублю (рублей за единицу иностранной валюты) (ежемесячные данные)","indicator_id":127,"indicator_parent":-1,"indicator_name":"Номинальный курс","link":"/statistics/macro_itm/external_sector/er/","begin_dt":2005,"end_dt":2026},{"category_id":33,"category_name":"Статистика внешнего сектора-Основные производные показатели динамики обменного курса рубля-Номинальные курсы иностранных валют к рублю (рублей за единицу иностранной валюты) (ежемесячные данные)","indicator_id":128,"indicator_parent":-1,"indicator_name":"Средний номинальный курс за период","link":"/statistics/macro_itm/external_sector/er/","begin_dt":2005,"end_dt":2026},{"category_id":33,"category_name":"Статистика внешнего сектора-Основные производные показатели динамики обменного курса рубля-Номинальные курсы иностранных валют к рублю (рублей за единицу иностранной валюты) (ежемесячные данные)","indicator_id":139,"indicator_parent":-1,"indicator_name":"Средний номинальный курс за период с начала года","link":"/statistics/macro_itm/external_sector/er/","begin_dt":2005,"end_dt":2026},{"category_id":35,"category_name":"Статистика внешнего сектора-Основные производные показатели динамики обменного курса рубля-Номинальные курсы иностранных валют к рублю (рублей за единицу иностранной валюты) (ежеквартальные данные)","indicator_id":133,"indicator_parent":-1,"indicator_name":"Номинальный курс","link":"/statistics/macro_itm/external_sector/er/","begin_dt":2005,"end_dt":2026},{"category_id":35,"category_name":"Статистика внешнего сектора-Основные производные показатели динамики обменного курса рубля-Номинальные курсы иностранных валют к рублю (рублей за единицу иностранной валюты) (ежеквартальные данные)","indicator_id":134,"indicator_parent":-1,"indicator_name":"Средний номинальный курс за период","link":"/statistics/macro_itm/external_sector/er/","begin_dt":2005,"end_dt":2026},{"category_id":35,"category_name":"Статистика внешнего сектора-Основные производные показатели динамики обменного курса рубля-Номинальные курсы иностранных валют к рублю (рублей за единицу иностранной валюты) (ежеквартальные данные)","indicator_id":141,"indicator_parent":-1,"indicator_name":"Средний номинальный курс за период с начала года","link":"/statistics/macro_itm/external_sector/er/","begin_dt":2005,"end_dt":2026}]}"#;
const DATA_NEW_JSON: &str = r#"{"RowData":[{"id":3204221,"indicator_id":7,"measure1_id":null,"measure2_id":22,"unit_id":3,"obs_val":15739.9,"date":"2021-08-01T00:00:00","periodicity":"month"}],"Links":[{"indicator_id":7,"indicator_parent":-1,"measure1_id":null,"measure2_id":12,"unit_id":3,"indicator_name":"M2","measure1_name":null,"measure2_name":"All","un_name":"bln RUB"}]}"#;

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct AdvancedPayload {
    value: i32,
    name: String,
}

#[derive(Debug, Serialize)]
struct AdvancedQuery {
    #[serde(rename = "indicatorId")]
    indicator_id: i32,
    page: i32,
}

fn publication_id(value: i32) -> PublicationId {
    PublicationId::new(value).unwrap()
}

fn dataset_id(value: i32) -> DatasetId {
    DatasetId::new(value).unwrap()
}

fn category_id(value: i32) -> CategoryId {
    CategoryId::new(value).unwrap()
}

fn indicator_id(value: i32) -> IndicatorId {
    IndicatorId::new(value).unwrap()
}

fn measure_id(value: i32) -> MeasureId {
    MeasureId::new(value).unwrap()
}

fn year_span(start: i32, end: i32) -> YearSpan {
    YearSpan::new(Year::new(start), Year::new(end)).unwrap()
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_client_supports_same_api() {
    use cbr_client::BlockingCbrClient;

    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/publications");
        then.status(200)
            .header("content-type", JSON_CONTENT_TYPE)
            .body(PUBLICATIONS_JSON);
    });

    let client = BlockingCbrClient::builder()
        .base_url(server.base_url())
        .build_blocking()
        .unwrap();
    let publications = client.publications().unwrap();

    assert_eq!(publications.len(), 1);
    mock.assert();
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_advanced_request_json_supports_custom_types() {
    use cbr_client::BlockingCbrClient;

    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/advanced-query")
            .query_param("indicatorId", "38")
            .query_param("page", "2");
        then.status(200)
            .header("content-type", JSON_CONTENT_TYPE)
            .body(r#"{"value":9,"name":"blocking"}"#);
    });

    let client = BlockingCbrClient::builder()
        .base_url(server.base_url())
        .build_blocking()
        .unwrap();
    let payload: AdvancedPayload = client
        .request_json_with_query(
            "/advanced-query",
            &AdvancedQuery {
                indicator_id: 38,
                page: 2,
            },
        )
        .unwrap();

    assert_eq!(
        payload,
        AdvancedPayload {
            value: 9,
            name: "blocking".to_owned(),
        }
    );
    mock.assert();
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_advanced_request_json_accepts_path_without_leading_slash() {
    use cbr_client::BlockingCbrClient;

    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/advanced-no-slash");
        then.status(200)
            .header("content-type", JSON_CONTENT_TYPE)
            .body(r#"{"value":12,"name":"blocking-no-slash"}"#);
    });

    let client = BlockingCbrClient::builder()
        .base_url(server.base_url())
        .build_blocking()
        .unwrap();
    let payload: AdvancedPayload = client.request_json("advanced-no-slash").unwrap();

    assert_eq!(
        payload,
        AdvancedPayload {
            value: 12,
            name: "blocking-no-slash".to_owned(),
        }
    );
    mock.assert();
}

#[cfg(feature = "blocking")]
#[test]
fn resolves_usd_rub_presets_from_catalog_blocking() {
    use cbr_client::BlockingCbrClient;

    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/categoryNew");
        then.status(200)
            .header("content-type", JSON_CONTENT_TYPE)
            .body(CATEGORY_NEW_FX_JSON);
    });

    let client = BlockingCbrClient::builder()
        .base_url(server.base_url())
        .build_blocking()
        .unwrap();
    let resolved_m_nominal =
        fx::resolve_fx_series_blocking(&client, FxPeriodicity::Monthly, FxMetric::Nominal)
            .unwrap()
            .unwrap();
    let resolved_m_avg =
        fx::resolve_fx_series_blocking(&client, FxPeriodicity::Monthly, FxMetric::Average)
            .unwrap()
            .unwrap();
    let resolved_q_nominal =
        fx::resolve_fx_series_blocking(&client, FxPeriodicity::Quarterly, FxMetric::Nominal)
            .unwrap()
            .unwrap();

    assert_eq!(resolved_m_nominal, fx::USD_RUB_MONTHLY_NOMINAL);
    assert_eq!(resolved_m_avg, fx::USD_RUB_MONTHLY_AVERAGE);
    assert_eq!(resolved_q_nominal, fx::USD_RUB_QUARTERLY_NOMINAL);
    mock.assert_calls(3);
}

#[test]
fn builder_rejects_invalid_proxy_url() {
    let error = CbrClient::builder()
        .proxy("://bad-proxy-url")
        .build()
        .unwrap_err();

    assert!(matches!(error, CbrError::Build(_)));

    #[cfg(feature = "blocking")]
    {
        let blocking_error = CbrClient::builder()
            .proxy("://bad-proxy-url")
            .build_blocking()
            .unwrap_err();

        assert!(matches!(blocking_error, CbrError::Build(_)));
    }
}
#[tokio::test]
async fn deserializes_all_endpoints() {
    let server = MockServer::start_async().await;

    let publications_mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/publications");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(PUBLICATIONS_JSON);
        })
        .await;
    let datasets_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/datasets")
                .query_param("publicationId", "18");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(DATASETS_JSON);
        })
        .await;
    let measures_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/measures")
                .query_param("datasetId", "38");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(MEASURES_JSON);
        })
        .await;
    let years_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/years")
                .query_param("datasetId", "38")
                .query_param("measureId", "2");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(YEARS_JSON);
        })
        .await;
    let years_ex_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/yearsEx")
                .query_param("publicationId", "18")
                .query_param("ids", "37")
                .query_param("ids", "38");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(YEARS_JSON);
        })
        .await;
    let datasets_ex_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/datasetsEx")
                .query_param("publicationId", "18");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(DATASETS_EX_JSON);
        })
        .await;
    let data_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/data")
                .query_param("y1", "2021")
                .query_param("y2", "2021")
                .query_param("datasetId", "38")
                .query_param("publicationId", "18")
                .query_param("measureId", "2");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(DATA_JSON);
        })
        .await;
    let data_ex_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/dataEx")
                .query_param("publicationId", "18")
                .query_param("y1", "2021")
                .query_param("y2", "2021")
                .query_param("i_ids", "38")
                .query_param("m1_ids", "2")
                .query_param("m2_ids", "9");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(DATA_EX_JSON);
        })
        .await;
    let dataset_description_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/DatasetDescription")
                .query_param("datasetId", "38");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(DATASET_DESCRIPTION_JSON);
        })
        .await;
    let category_new_mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/categoryNew");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(CATEGORY_NEW_JSON);
        })
        .await;
    let data_new_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/dataNew")
                .query_param("categoryId", "5")
                .query_param("y1", "2021")
                .query_param("y2", "2021")
                .query_param("i_ids", "7");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(DATA_NEW_JSON);
        })
        .await;

    let client = CbrClient::builder()
        .base_url(server.base_url())
        .build()
        .unwrap();

    let publications = client.publications().await.unwrap();
    assert_eq!(publications.len(), 1);
    assert!(publications[0].no_active);

    let datasets = client.datasets(publication_id(18)).await.unwrap();
    assert_eq!(datasets[0].id, dataset_id(37));
    assert!(matches!(datasets[0].kind, cbr_client::DatasetKind::Code1));
    assert!(matches!(
        datasets[0].reporting,
        cbr_client::DatasetReporting::Period
    ));

    let measures = client.measures(dataset_id(38)).await.unwrap();
    assert_eq!(measures.measure[0].name, "RUB");

    let years = client
        .years(dataset_id(38), Some(measure_id(2)))
        .await
        .unwrap();
    assert_eq!(years[0].from_year, Some(Year::new(2014)));

    let years_ex = client
        .years_ex(publication_id(18), &[dataset_id(37), dataset_id(38)])
        .await
        .unwrap();
    assert_eq!(years_ex[0].to_year, Some(Year::new(2025)));

    let datasets_ex = client.datasets_ex(publication_id(18)).await.unwrap();
    assert_eq!(datasets_ex.units[0].name, "%");

    let data = client
        .data(
            DataQuery::new(year_span(2021, 2021), dataset_id(38), publication_id(18))
                .with_measure_id(measure_id(2)),
        )
        .await
        .unwrap();
    assert_eq!(data.raw_data[0].obs_val, Some(3.31));

    let data_ex = client
        .data_ex(
            DataExQuery::new(publication_id(18), year_span(2021, 2021))
                .with_i_ids([indicator_id(38)])
                .with_m1_ids([measure_id(2)])
                .with_m2_ids([measure_id(9)]),
        )
        .await
        .unwrap();
    assert_eq!(data_ex.links[0].s_sort, Some(SortKey::Numeric(38)));

    let descriptions = client.dataset_description(dataset_id(38)).await.unwrap();
    assert_eq!(descriptions[0].description, "<p>Text</p>");

    let categories = client.category_new().await.unwrap();
    assert_eq!(categories.category[0].indicator_name, "M2");

    let data_new = client
        .data_new(
            DataNewQuery::new(category_id(5), year_span(2021, 2021)).with_i_ids([indicator_id(7)]),
        )
        .await
        .unwrap();
    assert_eq!(data_new.links[0].indicator_name.as_deref(), Some("M2"));
    assert!(data_new.row_data[0].measure1_id.is_none());
    assert!(data_new.links[0].measure1_name.is_none());

    publications_mock.assert_async().await;
    datasets_mock.assert_async().await;
    measures_mock.assert_async().await;
    years_mock.assert_async().await;
    years_ex_mock.assert_async().await;
    datasets_ex_mock.assert_async().await;
    data_mock.assert_async().await;
    data_ex_mock.assert_async().await;
    dataset_description_mock.assert_async().await;
    category_new_mock.assert_async().await;
    data_new_mock.assert_async().await;
}

#[tokio::test]
async fn serializes_array_params_as_repeated_keys() {
    let server = MockServer::start_async().await;

    let years_ex_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/yearsEx")
                .query_param("publicationId", "18")
                .query_param("ids", "37")
                .query_param("ids", "38")
                .query_param("ids", "39");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(YEARS_JSON);
        })
        .await;
    let data_ex_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/dataEx")
                .query_param("publicationId", "18")
                .query_param("y1", "2021")
                .query_param("y2", "2022")
                .query_param("i_ids", "37")
                .query_param("i_ids", "38")
                .query_param("m1_ids", "2")
                .query_param("m1_ids", "3")
                .query_param("m2_ids", "9")
                .query_param("m2_ids", "10");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(DATA_EX_JSON);
        })
        .await;
    let data_new_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/dataNew")
                .query_param("categoryId", "5")
                .query_param("y1", "2021")
                .query_param("y2", "2022")
                .query_param("i_ids", "7")
                .query_param("i_ids", "8")
                .query_param("m1_ids", "2")
                .query_param("m1_ids", "3")
                .query_param("m2_ids", "9")
                .query_param("m2_ids", "10");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(DATA_NEW_JSON);
        })
        .await;

    let client = CbrClient::builder()
        .base_url(server.base_url())
        .build()
        .unwrap();

    let _ = client
        .years_ex(
            publication_id(18),
            &[dataset_id(37), dataset_id(38), dataset_id(39)],
        )
        .await
        .unwrap();
    let _ = client
        .data_ex(
            DataExQuery::new(publication_id(18), year_span(2021, 2022))
                .with_i_ids([indicator_id(37), indicator_id(38)])
                .with_m1_ids([measure_id(2), measure_id(3)])
                .with_m2_ids([measure_id(9), measure_id(10)]),
        )
        .await
        .unwrap();
    let _ = client
        .data_new(
            DataNewQuery::new(category_id(5), year_span(2021, 2022))
                .with_i_ids([indicator_id(7), indicator_id(8)])
                .with_m1_ids([measure_id(2), measure_id(3)])
                .with_m2_ids([measure_id(9), measure_id(10)]),
        )
        .await
        .unwrap();

    years_ex_mock.assert_async().await;
    data_ex_mock.assert_async().await;
    data_new_mock.assert_async().await;
}

#[tokio::test]
async fn handles_legacy_error_payload() {
    let server = MockServer::start_async().await;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/publications");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body("{Error:true}");
        })
        .await;

    let client = CbrClient::builder()
        .base_url(server.base_url())
        .build()
        .unwrap();
    let error = client.publications().await.unwrap_err();
    assert!(matches!(
        error,
        CbrError::LegacyErrorResponse {
            payload_size: _,
            payload_preview: _
        }
    ));
    if let CbrError::LegacyErrorResponse {
        payload_preview,
        payload_size,
    } = error
    {
        assert_eq!(payload_size, "{Error:true}".len());
        assert_eq!(payload_preview, "{Error:true}");
    }

    mock.assert_async().await;
}

#[tokio::test]
async fn handles_legacy_error_payload_with_spaces_and_mixed_case() {
    let server = MockServer::start_async().await;

    let body = " {  eRrOr : TruE  } ";
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/publications");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(body);
        })
        .await;

    let client = CbrClient::builder()
        .base_url(server.base_url())
        .build()
        .unwrap();
    let error = client.publications().await.unwrap_err();
    assert!(matches!(error, CbrError::LegacyErrorResponse { .. }));
    mock.assert_async().await;
}

#[tokio::test]
async fn handles_non_success_status() {
    let server = MockServer::start_async().await;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/publications");
            then.status(503)
                .header("content-type", JSON_CONTENT_TYPE)
                .body("service unavailable");
        })
        .await;

    let client = CbrClient::builder()
        .base_url(server.base_url())
        .build()
        .unwrap();
    let error = client.publications().await.unwrap_err();
    if let CbrError::Status {
        status,
        body_preview,
        body_size,
    } = error
    {
        assert_eq!(status.as_u16(), 503);
        assert_eq!(body_size, "service unavailable".len());
        assert_eq!(body_preview, "service unavailable");
    } else {
        panic!("expected CbrError::Status");
    }
    mock.assert_async().await;
}

#[tokio::test]
async fn handles_invalid_json() {
    let server = MockServer::start_async().await;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/publications");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body("{not-json");
        })
        .await;

    let client = CbrClient::builder()
        .base_url(server.base_url())
        .build()
        .unwrap();
    let error = client.publications().await.unwrap_err();
    if let CbrError::Deserialize {
        body_preview,
        body_size,
        ..
    } = error
    {
        assert_eq!(body_size, "{not-json".len());
        assert_eq!(body_preview, "{not-json");
    } else {
        panic!("expected CbrError::Deserialize");
    }
    mock.assert_async().await;
}

#[tokio::test]
async fn resolves_usd_rub_presets_from_catalog() {
    let server = MockServer::start_async().await;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/categoryNew");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(CATEGORY_NEW_FX_JSON);
        })
        .await;

    let client = CbrClient::builder()
        .base_url(server.base_url())
        .build()
        .unwrap();

    let resolved_m_nominal =
        fx::resolve_fx_series(&client, FxPeriodicity::Monthly, FxMetric::Nominal)
            .await
            .unwrap()
            .unwrap();
    let resolved_m_avg = fx::resolve_fx_series(&client, FxPeriodicity::Monthly, FxMetric::Average)
        .await
        .unwrap()
        .unwrap();
    let resolved_m_avg_ytd =
        fx::resolve_fx_series(&client, FxPeriodicity::Monthly, FxMetric::AverageYtd)
            .await
            .unwrap()
            .unwrap();
    let resolved_q_nominal =
        fx::resolve_fx_series(&client, FxPeriodicity::Quarterly, FxMetric::Nominal)
            .await
            .unwrap()
            .unwrap();
    let resolved_q_avg =
        fx::resolve_fx_series(&client, FxPeriodicity::Quarterly, FxMetric::Average)
            .await
            .unwrap()
            .unwrap();
    let resolved_q_avg_ytd =
        fx::resolve_fx_series(&client, FxPeriodicity::Quarterly, FxMetric::AverageYtd)
            .await
            .unwrap()
            .unwrap();

    assert_eq!(resolved_m_nominal, fx::USD_RUB_MONTHLY_NOMINAL);
    assert_eq!(resolved_m_avg, fx::USD_RUB_MONTHLY_AVERAGE);
    assert_eq!(resolved_m_avg_ytd, fx::USD_RUB_MONTHLY_AVERAGE_YTD);
    assert_eq!(resolved_q_nominal, fx::USD_RUB_QUARTERLY_NOMINAL);
    assert_eq!(resolved_q_avg, fx::USD_RUB_QUARTERLY_AVERAGE);
    assert_eq!(resolved_q_avg_ytd, fx::USD_RUB_QUARTERLY_AVERAGE_YTD);
    mock.assert_calls_async(6).await;
}

#[tokio::test]
async fn advanced_request_json_supports_custom_response_types() {
    let server = MockServer::start_async().await;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/advanced");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(r#"{"value":42,"name":"ok"}"#);
        })
        .await;

    let client = CbrClient::builder()
        .base_url(server.base_url())
        .build()
        .unwrap();
    let payload: AdvancedPayload = client.request_json("/advanced").await.unwrap();

    assert_eq!(
        payload,
        AdvancedPayload {
            value: 42,
            name: "ok".to_owned(),
        }
    );
    mock.assert_async().await;
}

#[tokio::test]
async fn advanced_request_json_accepts_path_without_leading_slash() {
    let server = MockServer::start_async().await;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/advanced-no-slash");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(r#"{"value":11,"name":"no-slash"}"#);
        })
        .await;

    let client = CbrClient::builder()
        .base_url(server.base_url())
        .build()
        .unwrap();
    let payload: AdvancedPayload = client.request_json("advanced-no-slash").await.unwrap();

    assert_eq!(
        payload,
        AdvancedPayload {
            value: 11,
            name: "no-slash".to_owned(),
        }
    );
    mock.assert_async().await;
}

#[tokio::test]
async fn advanced_request_json_with_query_supports_custom_query_types() {
    let server = MockServer::start_async().await;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/advanced-query")
                .query_param("indicatorId", "38")
                .query_param("page", "2");
            then.status(200)
                .header("content-type", JSON_CONTENT_TYPE)
                .body(r#"{"value":7,"name":"page-2"}"#);
        })
        .await;

    let client = CbrClient::builder()
        .base_url(server.base_url())
        .build()
        .unwrap();
    let payload: AdvancedPayload = client
        .request_json_with_query(
            "/advanced-query",
            &AdvancedQuery {
                indicator_id: 38,
                page: 2,
            },
        )
        .await
        .unwrap();

    assert_eq!(
        payload,
        AdvancedPayload {
            value: 7,
            name: "page-2".to_owned(),
        }
    );
    mock.assert_async().await;
}
