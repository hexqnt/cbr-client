# cbr-client

[🇺🇸 English](./README.md) · [🇷🇺 Русский](./README.ru.md)

[![CI](https://github.com/hexqnt/cbr-client/actions/workflows/ci.yml/badge.svg)](https://github.com/hexqnt/cbr-client/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/cbr-client.svg)](https://crates.io/crates/cbr-client)
[![docs.rs](https://docs.rs/cbr-client/badge.svg)](https://docs.rs/cbr-client)

Неофициальная Rust-библиотека для работы с [API сервиса статистических данных Банка России](https://www.cbr.ru/statistics/data-service/APIdocumentation/).

## Установка

```toml
[dependencies]
cbr-client = "0.1.1"
```

## Асинхронный пример

```rust
use cbr_client::{CategoryId, CbrClient, DataNewQuery, IndicatorId, Year, YearSpan};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CbrClient::new()?;

    let years = YearSpan::new(Year::new(2021), Year::new(2021))?;
    let data = client
        .data_new(
            DataNewQuery::new(CategoryId::new(5)?, years)
                .with_i_ids([IndicatorId::new(7)?, IndicatorId::new(8)?]),
        )
        .await?;

    println!("rows: {}", data.row_data.len());
    Ok(())
}
```

## Блокирующий пример

Включите feature `blocking`:

```toml
[dependencies]
cbr-client = { version = "0.1.1", features = ["blocking"] }
```

```rust
use cbr_client::{
    BlockingCbrClient, DataQuery, DatasetId, MeasureId, PublicationId, Year, YearSpan,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BlockingCbrClient::new()?;
    let years = YearSpan::new(Year::new(2021), Year::new(2021))?;

    let data = client.data(
        DataQuery::new(years, DatasetId::new(38)?, PublicationId::new(18)?)
            .with_measure_id(MeasureId::new(2)?),
    )?;

    println!("raw rows: {}", data.raw_data.len());
    Ok(())
}
```

## Временной ряд USD/RUB

Полный пример находится в `examples/usd_rub_time_series.rs`. Модуль `cbr_client::presets::fx` предоставляет:

- Константные пресеты (`SeriesPreset`)
- Runtime-резолверы: `resolve_fx_series(client, FxPeriodicity, FxMetric)` и `resolve_fx_series_blocking(...)` (с feature `blocking`)

## Настройка клиента

```rust
use std::time::Duration;

use cbr_client::CbrClient;

let client = CbrClient::builder()
    .base_url("https://www.cbr.ru/dataservice")
    .timeout(Duration::from_secs(15))
    .user_agent("my-app/1.0")
    .build()?;
```

## Пользовательские типы ответов

Оба клиента предоставляют обобщённые методы `request_json<T>(path)` и `request_json_with_query<T, Q>(path, &query)` для десериализации ответов непосредственно в пользовательские типы. Пути задаются относительно `base_url`; ведущий `/` необязателен.

```rust
use cbr_client::CbrClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct MyPayload {
    value: i32,
}

#[derive(Debug, Serialize)]
struct MyQuery {
    page: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CbrClient::new()?;
    let _payload: MyPayload = client
        .request_json_with_query("/some-endpoint", &MyQuery { page: 1 })
        .await?;
    Ok(())
}
```

## Прокси

```rust
use cbr_client::CbrClient;

let client = CbrClient::builder()
    .proxy("http://127.0.0.1:8080")
    // Использовать настройки прокси из переменных окружения:
    // .use_system_proxy(true)
    .build()?;
```
