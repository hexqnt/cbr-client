use cbr_client::{
    CbrClient, DataNewQuery, IsoDateTime, Year, YearSpan,
    presets::fx::{self, FxMetric, FxPeriodicity},
};

const FROM_YEAR: i32 = 2020;
const TO_YEAR: i32 = 2025;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CbrClient::new()?;
    let preset = fx::resolve_fx_series(&client, FxPeriodicity::Monthly, FxMetric::Nominal)
        .await?
        .unwrap_or(fx::USD_RUB_MONTHLY_NOMINAL);

    let years = YearSpan::new(Year::new(FROM_YEAR), Year::new(TO_YEAR))?;
    let response = client
        .data_new(DataNewQuery::new(preset.category_id, years).with_i_ids([preset.indicator_id]))
        .await?;

    let usd_measure2_id = response
        .links
        .iter()
        .find(|link| link.measure2_id == Some(fx::USD_TO_RUB_END_OF_PERIOD_MEASURE2_ID))
        .and_then(|link| link.measure2_id)
        .or_else(|| {
            response
                .links
                .iter()
                .find(|link| {
                    link.indicator_id == Some(preset.indicator_id)
                        && link
                            .measure2_name
                            .as_deref()
                            .is_some_and(|name| name.contains("Доллара США к рублю"))
                })
                .and_then(|link| link.measure2_id)
        })
        .ok_or("USD/RUB measure was not found in dataNew response")?;

    let mut series: Vec<(IsoDateTime, f64)> = response
        .row_data
        .iter()
        .filter(|row| {
            row.indicator_id == Some(preset.indicator_id)
                && row.measure2_id == Some(usd_measure2_id)
        })
        .filter_map(|row| Some((row.date?, row.obs_val?)))
        .collect();

    series.sort_by_key(|item| item.0);

    println!("date,usd_rub");
    for (date, value) in series {
        println!("{date},{value:.4}");
    }

    Ok(())
}
