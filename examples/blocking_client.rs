#[cfg(feature = "blocking")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cbr_client::{
        BlockingCbrClient, DataQuery, DatasetId, MeasureId, PublicationId, Year, YearSpan,
    };

    let client = BlockingCbrClient::new()?;
    let years = YearSpan::new(Year::new(2021), Year::new(2021))?;
    let data = client.data(
        DataQuery::new(years, DatasetId::new(38)?, PublicationId::new(18)?)
            .with_measure_id(MeasureId::new(2)?),
    )?;

    println!("raw_data: {}", data.raw_data.len());
    println!("header_data: {}", data.header_data.len());
    Ok(())
}

#[cfg(not(feature = "blocking"))]
fn main() {
    eprintln!("Enable the `blocking` feature to run this example.");
}
