use std::fmt;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use crate::types::{
    CategoryId, ColumnId, DatasetId, DmyDate, ElementId, IndicatorId, InputError, IsoDateTime,
    MeasureId, ParentRef, PeriodId, Periodicity, PublicationId, RowId, UnitId, Year,
};

/// Индикатор в ответе `/datasetsEx`.
pub type DatasetExIndicator = NamedEntity<IndicatorId, IndicatorId>;

/// Разрез `measure_1` в ответе `/datasetsEx`.
pub type DatasetExMeasure1 = NamedEntity<MeasureId, IndicatorId>;

/// Разрез `measure_2` в ответе `/datasetsEx`.
pub type DatasetExMeasure2 = NamedEntity<MeasureId, IndicatorId>;

/// Единица измерения в ответе `/datasetsEx`.
pub type DatasetExUnit = NamedEntity<UnitId, UnitId>;

/// Тип кода показателя из `/datasets`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatasetKind {
    /// Код `1`.
    Code1,
    /// Код `2`.
    Code2,
    /// Неизвестный код.
    Unknown(i32),
}

impl DatasetKind {
    /// Создаёт тип из числового кода API.
    #[must_use]
    #[inline]
    pub fn from_code(value: i32) -> Self {
        match value {
            1 => Self::Code1,
            2 => Self::Code2,
            other => Self::Unknown(other),
        }
    }

    /// Возвращает исходный числовой код.
    #[must_use]
    #[inline]
    pub fn code(self) -> i32 {
        match self {
            Self::Code1 => 1,
            Self::Code2 => 2,
            Self::Unknown(value) => value,
        }
    }
}

impl<'de> Deserialize<'de> for DatasetKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_code(i32::deserialize(deserializer)?))
    }
}

/// Тип отчётности показателя (`reporting` в `/datasets`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatasetReporting {
    /// Значение `period`.
    Period,
    /// Любое неизвестное значение.
    Unknown(String),
}

impl DatasetReporting {
    /// Возвращает исходное строковое представление.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Period => "period",
            Self::Unknown(value) => value.as_str(),
        }
    }
}

impl<'de> Deserialize<'de> for DatasetReporting {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "period" => Self::Period,
            _ => Self::Unknown(value),
        })
    }
}

/// Код типа серии из `/data`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeriesType {
    /// Код `1`.
    Code1,
    /// Код `2`.
    Code2,
    /// Код `3`.
    Code3,
    /// Неизвестный код.
    Unknown(i32),
}

impl SeriesType {
    /// Создаёт тип из числового кода API.
    #[must_use]
    #[inline]
    pub fn from_code(value: i32) -> Self {
        match value {
            1 => Self::Code1,
            2 => Self::Code2,
            3 => Self::Code3,
            other => Self::Unknown(other),
        }
    }

    /// Возвращает исходный числовой код.
    #[must_use]
    #[inline]
    pub fn code(self) -> i32 {
        match self {
            Self::Code1 => 1,
            Self::Code2 => 2,
            Self::Code3 => 3,
            Self::Unknown(value) => value,
        }
    }
}

impl<'de> Deserialize<'de> for SeriesType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_code(i32::deserialize(deserializer)?))
    }
}

/// Ключ сортировки из `DataExLink::s_sort`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum SortKey {
    /// Числовой ключ сортировки.
    Numeric(i32),
    /// Текстовый ключ сортировки.
    Text(String),
}

impl SortKey {
    /// Возвращает числовое значение ключа, если оно доступно.
    #[must_use]
    #[inline]
    pub fn as_numeric(&self) -> Option<i32> {
        match self {
            Self::Numeric(value) => Some(*value),
            Self::Text(_) => None,
        }
    }

    /// Возвращает текстовое значение ключа, если оно нечисловое.
    #[must_use]
    #[inline]
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Numeric(_) => None,
            Self::Text(value) => Some(value.as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for SortKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SortKeyVisitor;

        impl Visitor<'_> for SortKeyVisitor {
            type Value = SortKey;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an integer or a string")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = i32::try_from(value)
                    .map_err(|_| E::custom(format!("sort key is out of i32 range: {value}")))?;
                Ok(SortKey::Numeric(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = i32::try_from(value)
                    .map_err(|_| E::custom(format!("sort key is out of i32 range: {value}")))?;
                Ok(SortKey::Numeric(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value.parse::<i32>() {
                    Ok(parsed) => Ok(SortKey::Numeric(parsed)),
                    Err(_) => Ok(SortKey::Text(value.to_owned())),
                }
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value.parse::<i32>() {
                    Ok(parsed) => Ok(SortKey::Numeric(parsed)),
                    Err(_) => Ok(SortKey::Text(value)),
                }
            }
        }

        deserializer.deserialize_any(SortKeyVisitor)
    }
}

/// Публикация (категория) из ответа `/publications`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Publication {
    pub id: PublicationId,
    pub parent_id: ParentRef<PublicationId>,
    pub category_name: String,
    #[serde(rename = "NoActive", deserialize_with = "deserialize_zero_one_bool")]
    pub no_active: bool,
    #[serde(default)]
    pub pub_description: Option<String>,
    #[serde(default)]
    pub updated_time: Option<IsoDateTime>,
}

/// Показатель из ответа `/datasets`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Dataset {
    pub id: DatasetId,
    pub parent_id: ParentRef<PublicationId>,
    pub name: String,
    pub full_name: String,
    #[serde(rename = "type")]
    pub kind: DatasetKind,
    pub reporting: DatasetReporting,
    pub link: String,
    #[serde(default)]
    pub updated_time: Option<IsoDateTime>,
}

/// Ответ метода `/measures`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct MeasuresResponse {
    /// Список разрезов показателя.
    pub measure: Vec<Measure>,
}

/// Разрез показателя.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Measure {
    pub id: MeasureId,
    pub parent_id: ParentRef<DatasetId>,
    pub name: String,
    #[serde(default)]
    pub sort: Option<i32>,
}

/// Диапазон годов.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct YearRange {
    #[serde(rename = "FromYear")]
    pub from_year: Option<Year>,
    #[serde(rename = "ToYear")]
    pub to_year: Option<Year>,
}

/// Расширенный справочник публикации из `/datasetsEx`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DatasetsExResponse {
    pub indicators: Vec<DatasetExIndicator>,
    #[serde(rename = "measures_1")]
    pub measures_1: Vec<DatasetExMeasure1>,
    #[serde(rename = "measures_2")]
    pub measures_2: Vec<DatasetExMeasure2>,
    pub units: Vec<DatasetExUnit>,
    pub years: Vec<YearRange>,
}

/// Универсальная именованная сущность из `/datasetsEx`.
#[derive(Debug, Clone, PartialEq)]
pub struct NamedEntity<Id, ParentId> {
    pub id: Id,
    pub parent_id: ParentRef<ParentId>,
    pub name: String,
}

impl<'de, Id, ParentId> Deserialize<'de> for NamedEntity<Id, ParentId>
where
    Id: Deserialize<'de>,
    ParentId: TryFrom<i32, Error = InputError> + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawNamedEntity<Id> {
            id: Id,
            parent_id: i32,
            name: String,
        }

        let raw = RawNamedEntity::deserialize(deserializer)?;
        Ok(Self {
            id: raw.id,
            parent_id: ParentRef::new(raw.parent_id).map_err(serde::de::Error::custom)?,
            name: raw.name,
        })
    }
}

/// Ответ метода `/data`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DataResponse {
    #[serde(rename = "RawData")]
    pub raw_data: Vec<DataRow>,
    #[serde(rename = "headerData")]
    pub header_data: Vec<DataHeader>,
    pub units: Vec<DataUnit>,
    #[serde(rename = "DTRange")]
    pub dt_range: Vec<DataRange>,
    #[serde(rename = "SType")]
    pub s_type: Vec<DataSeriesType>,
}

/// Строка данных из `DataResponse::raw_data`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DataRow {
    #[serde(rename = "colId")]
    pub col_id: Option<ColumnId>,
    pub element_id: Option<ElementId>,
    pub measure_id: Option<MeasureId>,
    pub unit_id: Option<UnitId>,
    pub obs_val: Option<f64>,
    #[serde(rename = "rowId")]
    pub row_id: Option<RowId>,
    pub dt: Option<String>,
    pub periodicity: Option<Periodicity>,
    pub date: Option<IsoDateTime>,
    pub digits: Option<i32>,
}

/// Заголовок колонки из `DataResponse::header_data`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DataHeader {
    pub id: Option<ColumnId>,
    pub elname: Option<String>,
}

/// Единица измерения из `DataResponse::units`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DataUnit {
    pub id: Option<UnitId>,
    pub val: Option<String>,
}

/// Диапазон дат из `DataResponse::dt_range`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DataRange {
    #[serde(rename = "FromY")]
    pub from_year: Option<Year>,
    #[serde(rename = "ToY")]
    pub to_year: Option<Year>,
}

/// Сведения о типе серии из `DataResponse::s_type`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DataSeriesType {
    #[serde(rename = "sType")]
    pub s_type: Option<SeriesType>,
    #[serde(rename = "dsName")]
    pub ds_name: Option<String>,
    #[serde(rename = "PublName")]
    pub publ_name: Option<String>,
}

/// Ответ метода `/dataEx`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DataExResponse {
    #[serde(rename = "RawData")]
    pub raw_data: Vec<DataExRow>,
    pub links: Vec<DataExLink>,
}

/// Строка данных из `DataExResponse::raw_data`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DataExRow {
    pub indicator_id: Option<IndicatorId>,
    pub measure_1_id: Option<MeasureId>,
    pub measure_2_id: Option<MeasureId>,
    pub unit_id: Option<UnitId>,
    pub value: Option<f64>,
    pub period_id: Option<PeriodId>,
    pub period: Option<String>,
    pub periodicity: Option<Periodicity>,
    pub date: Option<DmyDate>,
    #[serde(rename = "rowId")]
    pub row_id: Option<RowId>,
}

/// Справочные связи из `DataExResponse::links`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DataExLink {
    pub indicator_id: Option<IndicatorId>,
    pub measure_1_id: Option<MeasureId>,
    pub measure_2_id: Option<MeasureId>,
    pub unit_id: Option<UnitId>,
    pub name: Option<String>,
    #[serde(rename = "sSort")]
    pub s_sort: Option<SortKey>,
}

/// Описание (методология) показателя из `/DatasetDescription`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DatasetDescription {
    pub description: String,
}

/// Ответ метода `/categoryNew`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CategoryNewResponse {
    /// Список категорий и показателей.
    pub category: Vec<CategoryNewItem>,
}

/// Элемент списка категорий и показателей.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CategoryNewItem {
    pub category_id: CategoryId,
    pub category_name: String,
    pub indicator_id: IndicatorId,
    pub indicator_parent: ParentRef<IndicatorId>,
    pub indicator_name: String,
    pub link: String,
    pub begin_dt: Year,
    pub end_dt: Year,
}

/// Ответ метода `/dataNew`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DataNewResponse {
    #[serde(rename = "RowData")]
    pub row_data: Vec<DataNewRow>,
    #[serde(rename = "Links")]
    pub links: Vec<DataNewLink>,
}

/// Строка данных из `DataNewResponse::row_data`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DataNewRow {
    pub id: Option<RowId>,
    pub indicator_id: Option<IndicatorId>,
    pub measure1_id: Option<MeasureId>,
    pub measure2_id: Option<MeasureId>,
    pub unit_id: Option<UnitId>,
    pub obs_val: Option<f64>,
    pub date: Option<IsoDateTime>,
    pub periodicity: Option<Periodicity>,
}

/// Справочные связи из `DataNewResponse::links`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DataNewLink {
    pub indicator_id: Option<IndicatorId>,
    pub indicator_parent: Option<ParentRef<IndicatorId>>,
    pub measure1_id: Option<MeasureId>,
    pub measure2_id: Option<MeasureId>,
    pub unit_id: Option<UnitId>,
    pub indicator_name: Option<String>,
    pub measure1_name: Option<String>,
    pub measure2_name: Option<String>,
    pub un_name: Option<String>,
}

fn deserialize_zero_one_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match i32::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        value => Err(serde::de::Error::custom(format!(
            "expected 0 or 1 for boolean flag, got {value}"
        ))),
    }
}
