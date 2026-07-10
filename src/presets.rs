//! Предустановленные идентификаторы и резолверы для популярных рядов.
//!
//! Важно: значения ID в API ЦБ могут меняться со временем.
//! Поэтому, где возможно, используйте runtime-резолверы.

use crate::types::{CategoryId, IndicatorId};

/// Пресеты и резолверы для валютных рядов.
pub mod fx {
    #[cfg(feature = "blocking")]
    use crate::BlockingCbrClient;
    use crate::models::CategoryNewItem;
    use crate::types::{CategoryId, IndicatorId, MeasureId};
    use crate::{CbrClient, CbrError};

    use super::SeriesPreset;

    const CATEGORY_SUBSTR: &str = "Номинальные курсы иностранных валют к рублю";
    const MONTHLY_SUBSTR: &str = "ежемесячные данные";
    const QUARTERLY_SUBSTR: &str = "ежеквартальные данные";
    const INDICATOR_NOMINAL: &str = "Номинальный курс";
    const INDICATOR_AVERAGE: &str = "Средний номинальный курс за период";
    const INDICATOR_AVERAGE_YTD: &str = "Средний номинальный курс за период с начала года";

    /// Константный пресет ежемесячного номинального курса в категории курсов к RUB.
    ///
    /// Для выбора конкретной валюты используйте `measure2_id`-константы этого модуля.
    /// Актуален на дату реализации, но может устареть при изменениях API.
    pub const USD_RUB_MONTHLY_NOMINAL: SeriesPreset =
        SeriesPreset::new(CategoryId::new_const(33), IndicatorId::new_const(127));
    /// Константный пресет ежемесячного среднего курса за период.
    pub const USD_RUB_MONTHLY_AVERAGE: SeriesPreset =
        SeriesPreset::new(CategoryId::new_const(33), IndicatorId::new_const(128));
    /// Константный пресет ежемесячного среднего курса с начала года.
    pub const USD_RUB_MONTHLY_AVERAGE_YTD: SeriesPreset =
        SeriesPreset::new(CategoryId::new_const(33), IndicatorId::new_const(139));
    /// Константный пресет ежеквартального номинального курса.
    pub const USD_RUB_QUARTERLY_NOMINAL: SeriesPreset =
        SeriesPreset::new(CategoryId::new_const(35), IndicatorId::new_const(133));
    /// Константный пресет ежеквартального среднего курса за период.
    pub const USD_RUB_QUARTERLY_AVERAGE: SeriesPreset =
        SeriesPreset::new(CategoryId::new_const(35), IndicatorId::new_const(134));
    /// Константный пресет ежеквартального среднего курса с начала года.
    pub const USD_RUB_QUARTERLY_AVERAGE_YTD: SeriesPreset =
        SeriesPreset::new(CategoryId::new_const(35), IndicatorId::new_const(141));

    /// `measure2_id` для фильтра USD/RUB на конец периода.
    pub const USD_TO_RUB_END_OF_PERIOD_MEASURE2_ID: MeasureId = MeasureId::new_const(98);
    /// `measure2_id` для фильтра EUR/RUB на конец периода.
    pub const EUR_TO_RUB_END_OF_PERIOD_MEASURE2_ID: MeasureId = MeasureId::new_const(99);
    /// `measure2_id` для фильтра CNY/RUB на конец периода.
    pub const CNY_TO_RUB_END_OF_PERIOD_MEASURE2_ID: MeasureId = MeasureId::new_const(100);
    /// `measure2_id` для фильтра среднего USD/RUB за период.
    pub const USD_TO_RUB_AVERAGE_MEASURE2_ID: MeasureId = MeasureId::new_const(101);
    /// `measure2_id` для фильтра среднего EUR/RUB за период.
    pub const EUR_TO_RUB_AVERAGE_MEASURE2_ID: MeasureId = MeasureId::new_const(102);
    /// `measure2_id` для фильтра среднего CNY/RUB за период.
    pub const CNY_TO_RUB_AVERAGE_MEASURE2_ID: MeasureId = MeasureId::new_const(103);

    /// Тип периодичности валютного ряда.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FxPeriodicity {
        /// Ежемесячный ряд.
        Monthly,
        /// Ежеквартальный ряд.
        Quarterly,
    }

    /// Метрика валютного ряда.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FxMetric {
        /// Номинальный курс на конец периода.
        Nominal,
        /// Средний курс за период.
        Average,
        /// Средний курс с начала года.
        AverageYtd,
    }

    /// Блокирующая версия `resolve_fx_series`.
    ///
    /// Доступно только с feature `blocking`.
    #[cfg(feature = "blocking")]
    pub fn resolve_fx_series_blocking(
        client: &BlockingCbrClient,
        periodicity: FxPeriodicity,
        metric: FxMetric,
    ) -> Result<Option<SeriesPreset>, CbrError> {
        let response = client.category_new()?;
        Ok(find_fx_series(&response.category, periodicity, metric))
    }

    fn periodicity_substr(periodicity: FxPeriodicity) -> &'static str {
        match periodicity {
            FxPeriodicity::Monthly => MONTHLY_SUBSTR,
            FxPeriodicity::Quarterly => QUARTERLY_SUBSTR,
        }
    }

    fn indicator_name(metric: FxMetric) -> &'static str {
        match metric {
            FxMetric::Nominal => INDICATOR_NOMINAL,
            FxMetric::Average => INDICATOR_AVERAGE,
            FxMetric::AverageYtd => INDICATOR_AVERAGE_YTD,
        }
    }

    fn find_fx_series(
        items: &[CategoryNewItem],
        periodicity: FxPeriodicity,
        metric: FxMetric,
    ) -> Option<SeriesPreset> {
        let periodicity_substr = periodicity_substr(periodicity);
        let indicator_name = indicator_name(metric);

        items
            .iter()
            .find(|item| {
                item.category_name.contains(CATEGORY_SUBSTR)
                    && item.category_name.contains(periodicity_substr)
                    && item.indicator_name == indicator_name
            })
            .map(|item| SeriesPreset::new(item.category_id, item.indicator_id))
    }
    /// Пытается динамически найти пресет валютного ряда через каталог `/categoryNew`.
    ///
    /// Возвращает `Ok(None)`, если подходящая запись не найдена.
    pub async fn resolve_fx_series(
        client: &CbrClient,
        periodicity: FxPeriodicity,
        metric: FxMetric,
    ) -> Result<Option<SeriesPreset>, CbrError> {
        let response = client.category_new().await?;
        Ok(find_fx_series(&response.category, periodicity, metric))
    }
}
/// Пара идентификаторов категории и индикатора для запроса ряда через `/dataNew`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeriesPreset {
    /// Идентификатор категории (`categoryId`).
    pub category_id: CategoryId,
    /// Идентификатор индикатора (`i_ids`).
    pub indicator_id: IndicatorId,
}

impl SeriesPreset {
    /// Создаёт пресет ряда.
    #[must_use]
    #[inline]
    pub const fn new(category_id: CategoryId, indicator_id: IndicatorId) -> Self {
        Self {
            category_id,
            indicator_id,
        }
    }
}
