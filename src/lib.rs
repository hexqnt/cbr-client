//! Typed-клиент для API сервиса статистических данных Банка России.
//!
//! Базовый URL API по умолчанию: `https://www.cbr.ru/dataservice`.

/// Блокирующий клиент.
#[cfg(feature = "blocking")]
pub use blocking::BlockingCbrClient;
/// Асинхронный клиент, builder и базовый URL API по умолчанию.
pub use client::{CbrClient, CbrClientBuilder, DEFAULT_BASE_URL};
/// Единый тип ошибок библиотеки.
pub use error::CbrError;
/// Строго типизированные модели ответов API.
pub use models::*;
/// Предустановленные идентификаторы и runtime-резолверы.
pub use presets::SeriesPreset;
/// Типы запросов к endpoint с данными.
pub use query::{DataExQuery, DataNewQuery, DataQuery};
/// Строго типизированные входные значения и ошибки валидации.
pub use types::{
    CategoryId, ColumnId, DatasetId, DmyDate, ElementId, Id, IdKind, IndicatorId, InputError,
    IsoDateTime, MeasureId, ParentRef, PeriodId, Periodicity, PublicationId, RowId, UnitId, Year,
    YearSpan,
};

#[cfg(feature = "blocking")]
pub mod blocking;
pub mod client;
mod client_common;
pub mod error;
pub mod models;
pub mod presets;
pub mod query;
pub mod types;
