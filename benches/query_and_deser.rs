use cbr_client::{
    CategoryId, DataExQuery, DataExResponse, DataNewQuery, DataNewResponse, IndicatorId, MeasureId,
    PublicationId, Year, YearSpan,
};
use criterion::{Criterion, criterion_group, criterion_main};

const DATA_EX_JSON: &str = r#"{"RawData":[{"indicator_id":38,"measure_1_id":2,"measure_2_id":9,"unit_id":1,"value":4.6,"period_id":349,"period":"Jan 2021","periodicity":"month","date":"01.02.2021","rowId":2710568}],"links":[{"indicator_id":38,"measure_1_id":2,"measure_2_id":9,"unit_id":1,"name":"Name","sSort":"38"}]}"#;
const DATA_NEW_JSON: &str = r#"{"RowData":[{"id":3204221,"indicator_id":7,"measure1_id":null,"measure2_id":22,"unit_id":3,"obs_val":15739.9,"date":"2021-08-01T00:00:00","periodicity":"month"}],"Links":[{"indicator_id":7,"indicator_parent":-1,"measure1_id":null,"measure2_id":12,"unit_id":3,"indicator_name":"M2","measure1_name":null,"measure2_name":"All","un_name":"bln RUB"}]}"#;

fn years() -> YearSpan {
    YearSpan::new(Year::new(2021), Year::new(2022)).unwrap()
}

fn bench_query_serialization(c: &mut Criterion) {
    let data_ex = DataExQuery::new(PublicationId::new(18).unwrap(), years())
        .with_i_ids([IndicatorId::new(37).unwrap(), IndicatorId::new(38).unwrap()])
        .with_m1_ids([MeasureId::new(2).unwrap(), MeasureId::new(3).unwrap()])
        .with_m2_ids([MeasureId::new(9).unwrap(), MeasureId::new(10).unwrap()]);

    let data_new = DataNewQuery::new(CategoryId::new(5).unwrap(), years())
        .with_i_ids([IndicatorId::new(7).unwrap(), IndicatorId::new(8).unwrap()])
        .with_m1_ids([MeasureId::new(2).unwrap(), MeasureId::new(3).unwrap()])
        .with_m2_ids([MeasureId::new(9).unwrap(), MeasureId::new(10).unwrap()]);

    c.bench_function("serialize_data_ex_query", |b| {
        b.iter(|| serde_urlencoded::to_string(&data_ex).unwrap())
    });

    c.bench_function("serialize_data_new_query", |b| {
        b.iter(|| serde_urlencoded::to_string(&data_new).unwrap())
    });
}

fn bench_json_deserialization(c: &mut Criterion) {
    c.bench_function("deserialize_data_ex_response", |b| {
        b.iter(|| serde_json::from_str::<DataExResponse>(DATA_EX_JSON).unwrap())
    });

    c.bench_function("deserialize_data_new_response", |b| {
        b.iter(|| serde_json::from_str::<DataNewResponse>(DATA_NEW_JSON).unwrap())
    });
}

criterion_group!(
    benches,
    bench_query_serialization,
    bench_json_deserialization
);
criterion_main!(benches);
