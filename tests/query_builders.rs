use cbr_client::{
    CategoryId, DataExQuery, DataNewQuery, DataQuery, DatasetId, IndicatorId, MeasureId,
    PublicationId, Year, YearSpan,
};

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

#[test]
fn data_query_accessors_reflect_builder_state() {
    let years = year_span(2021, 2022);
    let base = DataQuery::new(years, dataset_id(38), publication_id(18));

    assert_eq!(base.years(), years);
    assert_eq!(base.dataset_id(), dataset_id(38));
    assert_eq!(base.publication_id(), publication_id(18));
    assert_eq!(base.measure_id(), None);

    let with_measure = base.clone().with_measure_id(measure_id(2));
    assert_eq!(with_measure.measure_id(), Some(measure_id(2)));
}

#[test]
fn data_ex_query_accessors_expose_id_filters() {
    let query = DataExQuery::new(publication_id(18), year_span(2021, 2022))
        .with_i_ids([indicator_id(37), indicator_id(38)])
        .with_m1_ids([measure_id(2)])
        .with_m2_ids([measure_id(9), measure_id(10)]);

    assert_eq!(query.publication_id(), publication_id(18));
    assert_eq!(query.years(), year_span(2021, 2022));
    assert_eq!(query.i_ids(), &[indicator_id(37), indicator_id(38)]);
    assert_eq!(query.m1_ids(), &[measure_id(2)]);
    assert_eq!(query.m2_ids(), &[measure_id(9), measure_id(10)]);
}

#[test]
fn data_new_query_accessors_expose_id_filters() {
    let query = DataNewQuery::new(category_id(5), year_span(2020, 2021))
        .with_i_ids([indicator_id(7), indicator_id(8)])
        .with_m1_ids([measure_id(2), measure_id(3)])
        .with_m2_ids([measure_id(9)]);

    assert_eq!(query.category_id(), category_id(5));
    assert_eq!(query.years(), year_span(2020, 2021));
    assert_eq!(query.i_ids(), &[indicator_id(7), indicator_id(8)]);
    assert_eq!(query.m1_ids(), &[measure_id(2), measure_id(3)]);
    assert_eq!(query.m2_ids(), &[measure_id(9)]);
}
