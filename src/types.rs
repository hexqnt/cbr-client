use std::{marker::PhantomData, num::NonZeroI32};

#[cfg(feature = "chrono")]
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::{Date, PrimitiveDateTime};

const ISO_DATETIME_FORMAT: &[time::format_description::FormatItem<'static>] =
    time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
const DMY_DATE_FORMAT: &[time::format_description::FormatItem<'static>] =
    time::macros::format_description!("[day].[month].[year]");

/// Строго типизированный идентификатор публикации.
pub type PublicationId = Id<PublicationIdKind>;

/// Строго типизированный идентификатор показателя (`dataset`).
pub type DatasetId = Id<DatasetIdKind>;

/// Строго типизированный идентификатор категории.
pub type CategoryId = Id<CategoryIdKind>;

/// Строго типизированный идентификатор индикатора.
pub type IndicatorId = Id<IndicatorIdKind>;

/// Строго типизированный идентификатор разреза (`measure`).
pub type MeasureId = Id<MeasureIdKind>;

/// Строго типизированный идентификатор единицы измерения.
pub type UnitId = Id<UnitIdKind>;

/// Строго типизированный идентификатор строки данных.
pub type RowId = Id<RowIdKind>;

/// Строго типизированный идентификатор периода.
pub type PeriodId = Id<PeriodIdKind>;

/// Строго типизированный идентификатор колонки.
pub type ColumnId = Id<ColumnIdKind>;

/// Строго типизированный идентификатор элемента.
pub type ElementId = Id<ElementIdKind>;

/// Ошибки валидации входных параметров.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InputError {
    /// Идентификатор должен быть строго положительным.
    #[error("{kind} must be strictly positive, got {value}")]
    NonPositiveId { kind: &'static str, value: i32 },
    /// Левая граница периода больше правой.
    #[error("year span start {start} must be less than or equal to end {end}")]
    InvalidYearSpan { start: i32, end: i32 },
    /// Родительская ссылка должна быть `-1` (корень) или положительным id.
    #[error("parent reference must be -1 (root) or positive id, got {value}")]
    InvalidParentRef { value: i32 },
    /// Значение chrono выходит за поддерживаемый диапазон.
    #[cfg(feature = "chrono")]
    #[error("chrono value is out of range for {kind}")]
    ChronoOutOfRange { kind: &'static str },
    /// Для ISO-формата поддерживается только точность до секунд.
    #[cfg(feature = "chrono")]
    #[error("sub-second precision is not supported for chrono conversion")]
    ChronoSubsecondPrecision,
}

#[doc(hidden)]
pub enum PublicationIdKind {}
impl IdKind for PublicationIdKind {
    const NAME: &'static str = "publication_id";
}
#[doc(hidden)]
pub enum DatasetIdKind {}
impl IdKind for DatasetIdKind {
    const NAME: &'static str = "dataset_id";
}
#[doc(hidden)]
pub enum CategoryIdKind {}
impl IdKind for CategoryIdKind {
    const NAME: &'static str = "category_id";
}
#[doc(hidden)]
pub enum IndicatorIdKind {}
impl IdKind for IndicatorIdKind {
    const NAME: &'static str = "indicator_id";
}
#[doc(hidden)]
pub enum MeasureIdKind {}
impl IdKind for MeasureIdKind {
    const NAME: &'static str = "measure_id";
}
#[doc(hidden)]
pub enum UnitIdKind {}
impl IdKind for UnitIdKind {
    const NAME: &'static str = "unit_id";
}
#[doc(hidden)]
pub enum RowIdKind {}
impl IdKind for RowIdKind {
    const NAME: &'static str = "row_id";
}
#[doc(hidden)]
pub enum PeriodIdKind {}
impl IdKind for PeriodIdKind {
    const NAME: &'static str = "period_id";
}
#[doc(hidden)]
pub enum ColumnIdKind {}
impl IdKind for ColumnIdKind {
    const NAME: &'static str = "column_id";
}
#[doc(hidden)]
pub enum ElementIdKind {}
impl IdKind for ElementIdKind {
    const NAME: &'static str = "element_id";
}
/// Родительская ссылка с явным корнем (`-1`) или валидным id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParentRef<T> {
    /// Корневой элемент (`-1` в API).
    Root,
    /// Ссылка на родительский элемент.
    Id(T),
}

impl<T> ParentRef<T>
where
    T: TryFrom<i32, Error = InputError> + Copy,
{
    /// Создаёт ссылку из сырого значения API.
    pub fn new(value: i32) -> Result<Self, InputError> {
        match value {
            -1 => Ok(Self::Root),
            value if value <= 0 => Err(InputError::InvalidParentRef { value }),
            _ => T::try_from(value).map(Self::Id),
        }
    }

    /// Возвращает `true`, если это корневая ссылка.
    #[must_use]
    #[inline]
    pub fn is_root(self) -> bool {
        matches!(self, Self::Root)
    }
}

impl<T: Copy> ParentRef<T> {
    /// Возвращает id родителя, если ссылка не корневая.
    #[must_use]
    #[inline]
    pub fn id(self) -> Option<T> {
        match self {
            Self::Root => None,
            Self::Id(value) => Some(value),
        }
    }
}

impl<T> Serialize for ParentRef<T>
where
    T: Copy + Into<i32>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let raw = match self {
            Self::Root => -1,
            Self::Id(value) => (*value).into(),
        };
        serializer.serialize_i32(raw)
    }
}

impl<'de, T> Deserialize<'de> for ParentRef<T>
where
    T: TryFrom<i32, Error = InputError> + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = i32::deserialize(deserializer)?;
        Self::new(raw).map_err(serde::de::Error::custom)
    }
}

/// Периодичность ряда.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Periodicity {
    /// Месячная периодичность.
    Month,
    /// Квартальная периодичность.
    Quarter,
    /// Годовая периодичность.
    Year,
}

/// Обобщённый строго типизированный идентификатор.
pub struct Id<K>(NonZeroI32, PhantomData<K>);

impl<K: IdKind> Id<K> {
    /// Создаёт идентификатор из целого числа.
    pub fn new(value: i32) -> Result<Self, InputError> {
        if value <= 0 {
            return Err(InputError::NonPositiveId {
                kind: K::NAME,
                value,
            });
        }

        let raw =
            NonZeroI32::new(value).expect("strictly positive value always produces NonZeroI32");
        Ok(Self(raw, PhantomData))
    }

    /// Создаёт идентификатор в `const`-контексте.
    ///
    /// Паникует на этапе компиляции, если `value <= 0`.
    #[must_use]
    #[inline]
    pub const fn new_const(value: i32) -> Self {
        if value <= 0 {
            panic!("identifier must be strictly positive");
        }

        match NonZeroI32::new(value) {
            Some(raw) => Self(raw, PhantomData),
            None => panic!("identifier must be strictly positive"),
        }
    }

    /// Возвращает исходное числовое значение идентификатора.
    #[must_use]
    #[inline]
    pub fn get(self) -> i32 {
        self.0.get()
    }
}

impl<K> Copy for Id<K> {}

impl<K> Clone for Id<K> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K> PartialEq for Id<K> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K> Eq for Id<K> {}

impl<K> PartialOrd for Id<K> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K> Ord for Id<K> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<K: IdKind> TryFrom<i32> for Id<K> {
    type Error = InputError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<K> std::fmt::Debug for Id<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Id").field(&self.0.get()).finish()
    }
}

impl<K> std::hash::Hash for Id<K> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<K: IdKind> Serialize for Id<K> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.get())
    }
}

impl<'de, K: IdKind> Deserialize<'de> for Id<K> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = i32::deserialize(deserializer)?;
        Self::new(raw).map_err(serde::de::Error::custom)
    }
}

/// Календарный год.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Year(i32);

impl Year {
    /// Создаёт значение года.
    #[must_use]
    #[inline]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Возвращает исходное значение года.
    #[must_use]
    #[inline]
    pub fn get(self) -> i32 {
        self.0
    }
}

/// Дата и время в формате `YYYY-MM-DDTHH:MM:SS`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IsoDateTime(PrimitiveDateTime);

impl IsoDateTime {
    /// Создаёт значение из `time::PrimitiveDateTime`.
    #[must_use]
    #[inline]
    pub const fn new(value: PrimitiveDateTime) -> Self {
        Self(value)
    }

    /// Парсит строку формата `YYYY-MM-DDTHH:MM:SS`.
    pub fn parse(value: &str) -> Result<Self, time::error::Parse> {
        PrimitiveDateTime::parse(value, ISO_DATETIME_FORMAT).map(Self)
    }

    /// Возвращает внутреннее представление.
    #[must_use]
    #[inline]
    pub const fn get(self) -> PrimitiveDateTime {
        self.0
    }

    /// Конвертирует значение `chrono::NaiveDateTime` в `IsoDateTime`.
    #[cfg(feature = "chrono")]
    pub fn try_from_chrono(value: NaiveDateTime) -> Result<Self, InputError> {
        Self::try_from(value)
    }

    /// Конвертирует в `chrono::NaiveDateTime`.
    #[cfg(feature = "chrono")]
    pub fn try_to_chrono(self) -> Result<NaiveDateTime, InputError> {
        let date = self.0.date();
        let chrono_date = NaiveDate::from_ymd_opt(
            date.year(),
            u32::from(u8::from(date.month())),
            u32::from(date.day()),
        )
        .ok_or(InputError::ChronoOutOfRange { kind: "date" })?;

        let time = self.0.time();
        chrono_date
            .and_hms_nano_opt(
                u32::from(time.hour()),
                u32::from(time.minute()),
                u32::from(time.second()),
                time.nanosecond(),
            )
            .ok_or(InputError::ChronoOutOfRange { kind: "datetime" })
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<NaiveDateTime> for IsoDateTime {
    type Error = InputError;

    fn try_from(value: NaiveDateTime) -> Result<Self, Self::Error> {
        if value.nanosecond() != 0 {
            return Err(InputError::ChronoSubsecondPrecision);
        }

        let month = chrono_month_to_time(value.month())?;
        let day =
            u8::try_from(value.day()).map_err(|_| InputError::ChronoOutOfRange { kind: "day" })?;
        let date = Date::from_calendar_date(value.year(), month, day)
            .map_err(|_| InputError::ChronoOutOfRange { kind: "date" })?;
        let hour = u8::try_from(value.hour())
            .map_err(|_| InputError::ChronoOutOfRange { kind: "hour" })?;
        let minute = u8::try_from(value.minute())
            .map_err(|_| InputError::ChronoOutOfRange { kind: "minute" })?;
        let second = u8::try_from(value.second())
            .map_err(|_| InputError::ChronoOutOfRange { kind: "second" })?;
        let time = time::Time::from_hms(hour, minute, second)
            .map_err(|_| InputError::ChronoOutOfRange { kind: "time" })?;

        Ok(Self::new(PrimitiveDateTime::new(date, time)))
    }
}

impl std::fmt::Display for IsoDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self
            .0
            .format(ISO_DATETIME_FORMAT)
            .map_err(|_| std::fmt::Error)?;
        f.write_str(&value)
    }
}

impl Serialize for IsoDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = self
            .0
            .format(ISO_DATETIME_FORMAT)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&value)
    }
}

impl<'de> Deserialize<'de> for IsoDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <&str>::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

/// Календарная дата в формате `DD.MM.YYYY`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DmyDate(Date);

impl DmyDate {
    /// Создаёт значение из `time::Date`.
    #[must_use]
    #[inline]
    pub const fn new(value: Date) -> Self {
        Self(value)
    }

    /// Парсит строку формата `DD.MM.YYYY`.
    pub fn parse(value: &str) -> Result<Self, time::error::Parse> {
        Date::parse(value, DMY_DATE_FORMAT).map(Self)
    }

    /// Возвращает внутреннее представление.
    #[must_use]
    #[inline]
    pub const fn get(self) -> Date {
        self.0
    }

    /// Конвертирует значение `chrono::NaiveDate` в `DmyDate`.
    #[cfg(feature = "chrono")]
    pub fn try_from_chrono(value: NaiveDate) -> Result<Self, InputError> {
        Self::try_from(value)
    }

    /// Конвертирует в `chrono::NaiveDate`.
    #[cfg(feature = "chrono")]
    pub fn try_to_chrono(self) -> Result<NaiveDate, InputError> {
        let date = self.0;
        NaiveDate::from_ymd_opt(
            date.year(),
            u32::from(u8::from(date.month())),
            u32::from(date.day()),
        )
        .ok_or(InputError::ChronoOutOfRange { kind: "date" })
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<NaiveDate> for DmyDate {
    type Error = InputError;

    fn try_from(value: NaiveDate) -> Result<Self, Self::Error> {
        let month = chrono_month_to_time(value.month())?;
        let day =
            u8::try_from(value.day()).map_err(|_| InputError::ChronoOutOfRange { kind: "day" })?;
        Date::from_calendar_date(value.year(), month, day)
            .map(Self::new)
            .map_err(|_| InputError::ChronoOutOfRange { kind: "date" })
    }
}

impl std::fmt::Display for DmyDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self
            .0
            .format(DMY_DATE_FORMAT)
            .map_err(|_| std::fmt::Error)?;
        f.write_str(&value)
    }
}

impl Serialize for DmyDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = self
            .0
            .format(DMY_DATE_FORMAT)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&value)
    }
}

impl<'de> Deserialize<'de> for DmyDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <&str>::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

/// Диапазон годов включительно.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct YearSpan {
    start: Year,
    end: Year,
}

impl YearSpan {
    /// Создаёт диапазон годов с проверкой `start <= end`.
    pub fn new(start: Year, end: Year) -> Result<Self, InputError> {
        if start > end {
            return Err(InputError::InvalidYearSpan {
                start: start.get(),
                end: end.get(),
            });
        }

        Ok(Self { start, end })
    }

    /// Левая граница диапазона.
    #[must_use]
    #[inline]
    pub fn start(self) -> Year {
        self.start
    }

    /// Правая граница диапазона.
    #[must_use]
    #[inline]
    pub fn end(self) -> Year {
        self.end
    }
}
/// Маркер домена идентификатора.
pub trait IdKind {
    /// Имя поля для текста ошибки.
    const NAME: &'static str;
}

impl<K: IdKind> From<Id<K>> for i32 {
    fn from(value: Id<K>) -> Self {
        value.get()
    }
}

impl From<Year> for i32 {
    fn from(value: Year) -> Self {
        value.get()
    }
}

#[cfg(feature = "chrono")]
fn chrono_month_to_time(value: u32) -> Result<time::Month, InputError> {
    let month = u8::try_from(value).map_err(|_| InputError::ChronoOutOfRange { kind: "month" })?;
    time::Month::try_from(month).map_err(|_| InputError::ChronoOutOfRange { kind: "month" })
}
