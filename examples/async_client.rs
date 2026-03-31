use cbr_client::{CategoryId, CbrClient, DataNewQuery, IndicatorId, Year, YearSpan};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CbrClient::new()?;

    let publications = client.publications().await?;
    println!("publications: {}", publications.len());

    let years = YearSpan::new(Year::new(2021), Year::new(2021))?;
    let category_id = CategoryId::new(5)?;
    let indicator_id_1 = IndicatorId::new(7)?;
    let indicator_id_2 = IndicatorId::new(8)?;

    let data = client
        .data_new(
            DataNewQuery::new(category_id, years).with_i_ids([indicator_id_1, indicator_id_2]),
        )
        .await?;
    println!("row_data: {}", data.row_data.len());
    println!("links: {}", data.links.len());

    Ok(())
}
