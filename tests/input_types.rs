use cbr_client::{InputError, ParentRef, PublicationId, Year, YearSpan};

#[cfg(feature = "chrono")]
use cbr_client::{DmyDate, IsoDateTime};

#[test]
fn id_rejects_zero() {
    let error = PublicationId::new(0).unwrap_err();
    assert!(matches!(error, InputError::NonPositiveId { value: 0, .. }));
}

#[test]
fn id_rejects_negative() {
    let error = PublicationId::new(-1).unwrap_err();
    assert!(matches!(error, InputError::NonPositiveId { value: -1, .. }));
}

#[test]
fn id_accepts_positive() {
    assert_eq!(PublicationId::new(1).unwrap().get(), 1);
}

#[test]
fn parent_ref_accepts_root_and_positive_id() {
    let root = ParentRef::<PublicationId>::new(-1).unwrap();
    let parent = ParentRef::<PublicationId>::new(42).unwrap();

    assert!(matches!(root, ParentRef::Root));
    assert_eq!(parent.id(), Some(PublicationId::new(42).unwrap()));
}

#[test]
fn parent_ref_rejects_zero_and_negative_values() {
    let zero_error = ParentRef::<PublicationId>::new(0).unwrap_err();
    let negative_error = ParentRef::<PublicationId>::new(-2).unwrap_err();

    assert!(matches!(
        zero_error,
        InputError::InvalidParentRef { value: 0 }
    ));
    assert!(matches!(
        negative_error,
        InputError::InvalidParentRef { value: -2 }
    ));
}

#[test]
fn year_span_rejects_descending_range() {
    let start = Year::new(2026);
    let end = Year::new(2025);

    let error = YearSpan::new(start, end).unwrap_err();
    assert!(matches!(error, InputError::InvalidYearSpan { .. }));
}

#[test]
fn year_span_accepts_valid_range() {
    let span = YearSpan::new(Year::new(2020), Year::new(2025)).unwrap();

    assert_eq!(span.start().get(), 2020);
    assert_eq!(span.end().get(), 2025);
}

#[cfg(feature = "chrono")]
#[test]
fn chrono_conversion_roundtrip_for_iso_datetime() {
    let chrono_dt = chrono::NaiveDate::from_ymd_opt(2026, 3, 16)
        .unwrap()
        .and_hms_opt(12, 30, 45)
        .unwrap();

    let value = IsoDateTime::try_from_chrono(chrono_dt).unwrap();
    let roundtrip = value.try_to_chrono().unwrap();

    assert_eq!(roundtrip, chrono_dt);
}

#[cfg(feature = "chrono")]
#[test]
fn chrono_conversion_rejects_subseconds() {
    let chrono_dt = chrono::NaiveDate::from_ymd_opt(2026, 3, 16)
        .unwrap()
        .and_hms_milli_opt(12, 30, 45, 123)
        .unwrap();

    let error = IsoDateTime::try_from_chrono(chrono_dt).unwrap_err();
    assert!(matches!(error, InputError::ChronoSubsecondPrecision));
}

#[cfg(feature = "chrono")]
#[test]
fn chrono_conversion_roundtrip_for_dmy_date() {
    let chrono_date = chrono::NaiveDate::from_ymd_opt(2026, 3, 16).unwrap();

    let value = DmyDate::try_from_chrono(chrono_date).unwrap();
    let roundtrip = value.try_to_chrono().unwrap();

    assert_eq!(roundtrip, chrono_date);
}
