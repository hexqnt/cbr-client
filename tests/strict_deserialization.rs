use cbr_client::{CategoryNewResponse, DataExResponse, DataNewResponse, Publication, SortKey};

#[test]
fn rejects_zero_id_in_publication() {
    let json = r#"[{"id":0,"parent_id":-1,"category_name":"Rates","NoActive":1}]"#;
    assert!(serde_json::from_str::<Vec<Publication>>(json).is_err());
}

#[test]
fn rejects_invalid_iso_datetime_format() {
    let json = r#"{
        "RowData":[{"id":3204221,"indicator_id":7,"measure1_id":null,"measure2_id":22,"unit_id":3,"obs_val":15739.9,"date":"2021/08/01","periodicity":"month"}],
        "Links":[{"indicator_id":7,"indicator_parent":-1,"measure1_id":null,"measure2_id":12,"unit_id":3,"indicator_name":"M2","measure1_name":null,"measure2_name":"All","un_name":"bln RUB"}]
    }"#;
    assert!(serde_json::from_str::<DataNewResponse>(json).is_err());
}

#[test]
fn rejects_invalid_dmy_date_format() {
    let json = r#"{
        "RawData":[{"indicator_id":38,"measure_1_id":2,"measure_2_id":9,"unit_id":1,"value":4.6,"period_id":349,"period":"Jan 2021","periodicity":"month","date":"2021-02-01","rowId":2710568}],
        "links":[{"indicator_id":38,"measure_1_id":2,"measure_2_id":9,"unit_id":1,"name":"Name","sSort":"38"}]
    }"#;
    assert!(serde_json::from_str::<DataExResponse>(json).is_err());
}

#[test]
fn rejects_unknown_periodicity() {
    let json = r#"{
        "RowData":[{"id":3204221,"indicator_id":7,"measure1_id":null,"measure2_id":22,"unit_id":3,"obs_val":15739.9,"date":"2021-08-01T00:00:00","periodicity":"weekly"}],
        "Links":[{"indicator_id":7,"indicator_parent":-1,"measure1_id":null,"measure2_id":12,"unit_id":3,"indicator_name":"M2","measure1_name":null,"measure2_name":"All","un_name":"bln RUB"}]
    }"#;
    assert!(serde_json::from_str::<DataNewResponse>(json).is_err());
}

#[test]
fn rejects_zero_parent_reference() {
    let json = r#"{
        "category":[{"category_id":5,"category_name":"Money","indicator_id":7,"indicator_parent":0,"indicator_name":"M2","link":"/x","begin_dt":1992,"end_dt":2026}]
    }"#;
    assert!(serde_json::from_str::<CategoryNewResponse>(json).is_err());
}

#[test]
fn rejects_non_numeric_parent_reference() {
    let json = r#"{
        "category":[{"category_id":5,"category_name":"Money","indicator_id":7,"indicator_parent":"oops","indicator_name":"M2","link":"/x","begin_dt":1992,"end_dt":2026}]
    }"#;
    assert!(serde_json::from_str::<CategoryNewResponse>(json).is_err());
}

#[test]
fn accepts_sort_key_from_string_number() {
    let json = r#"{
        "RawData":[{"indicator_id":38,"measure_1_id":2,"measure_2_id":9,"unit_id":1,"value":4.6,"period_id":349,"period":"Jan 2021","periodicity":"month","date":"01.02.2021","rowId":2710568}],
        "links":[{"indicator_id":38,"measure_1_id":2,"measure_2_id":9,"unit_id":1,"name":"Name","sSort":"38"}]
    }"#;

    let response = serde_json::from_str::<DataExResponse>(json).unwrap();
    assert_eq!(response.links[0].s_sort, Some(SortKey::Numeric(38)));
}

#[test]
fn accepts_sort_key_from_json_number() {
    let json = r#"{
        "RawData":[{"indicator_id":38,"measure_1_id":2,"measure_2_id":9,"unit_id":1,"value":4.6,"period_id":349,"period":"Jan 2021","periodicity":"month","date":"01.02.2021","rowId":2710568}],
        "links":[{"indicator_id":38,"measure_1_id":2,"measure_2_id":9,"unit_id":1,"name":"Name","sSort":38}]
    }"#;

    let response = serde_json::from_str::<DataExResponse>(json).unwrap();
    assert_eq!(response.links[0].s_sort, Some(SortKey::Numeric(38)));
}
