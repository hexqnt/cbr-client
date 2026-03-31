use serde::Serialize;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};

use crate::types::{CategoryId, DatasetId, IndicatorId, MeasureId, PublicationId, Year, YearSpan};

#[derive(Debug, Clone, Copy)]
struct YearsRangeQuery {
    span: YearSpan,
}

impl YearsRangeQuery {
    #[inline]
    fn from_span(span: YearSpan) -> Self {
        Self { span }
    }

    #[inline]
    fn span(self) -> YearSpan {
        self.span
    }

    #[inline]
    fn start(self) -> Year {
        self.span.start()
    }

    #[inline]
    fn end(self) -> Year {
        self.span.end()
    }
}

impl Serialize for YearsRangeQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("y1", &self.start())?;
        map.serialize_entry("y2", &self.end())?;
        map.end()
    }
}

/// Параметры для метода `/data`.
#[derive(Debug, Clone)]
pub struct DataQuery {
    years: YearsRangeQuery,
    dataset_id: DatasetId,
    publication_id: PublicationId,
    measure_id: Option<MeasureId>,
}

impl DataQuery {
    /// Создаёт запрос с обязательными полями.
    #[must_use]
    #[inline]
    pub fn new(years: YearSpan, dataset_id: DatasetId, publication_id: PublicationId) -> Self {
        Self {
            years: YearsRangeQuery::from_span(years),
            dataset_id,
            publication_id,
            measure_id: None,
        }
    }

    /// Добавляет параметр `measureId`.
    #[must_use]
    #[inline]
    pub fn with_measure_id(mut self, measure_id: MeasureId) -> Self {
        self.measure_id = Some(measure_id);
        self
    }

    /// Возвращает диапазон годов запроса.
    #[must_use]
    #[inline]
    pub fn years(&self) -> YearSpan {
        self.years.span()
    }

    /// Возвращает идентификатор показателя.
    #[must_use]
    #[inline]
    pub fn dataset_id(&self) -> DatasetId {
        self.dataset_id
    }

    /// Возвращает идентификатор публикации.
    #[must_use]
    #[inline]
    pub fn publication_id(&self) -> PublicationId {
        self.publication_id
    }

    /// Возвращает идентификатор разреза, если задан.
    #[must_use]
    #[inline]
    pub fn measure_id(&self) -> Option<MeasureId> {
        self.measure_id
    }
}

impl Serialize for DataQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let fields = if self.measure_id.is_some() { 5 } else { 4 };
        let mut map = serializer.serialize_map(Some(fields))?;
        map.serialize_entry("y1", &self.years.start())?;
        map.serialize_entry("y2", &self.years.end())?;
        map.serialize_entry("datasetId", &self.dataset_id)?;
        map.serialize_entry("publicationId", &self.publication_id)?;

        if let Some(measure_id) = self.measure_id {
            map.serialize_entry("measureId", &measure_id)?;
        }

        map.end()
    }
}

/// Параметры для метода `/dataEx`.
#[derive(Debug, Clone)]
pub struct DataExQuery {
    inner: MultiIdsQuery<PublicationId>,
}

impl DataExQuery {
    /// Создаёт запрос с обязательными полями.
    #[must_use]
    #[inline]
    pub fn new(publication_id: PublicationId, years: YearSpan) -> Self {
        Self {
            inner: MultiIdsQuery::new(publication_id, years),
        }
    }

    /// Устанавливает массив `i_ids`.
    #[must_use]
    #[inline]
    pub fn with_i_ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = IndicatorId>,
    {
        self.inner = self.inner.with_i_ids(ids);
        self
    }

    /// Устанавливает массив `m1_ids`.
    #[must_use]
    #[inline]
    pub fn with_m1_ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = MeasureId>,
    {
        self.inner = self.inner.with_m1_ids(ids);
        self
    }

    /// Устанавливает массив `m2_ids`.
    #[must_use]
    #[inline]
    pub fn with_m2_ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = MeasureId>,
    {
        self.inner = self.inner.with_m2_ids(ids);
        self
    }

    /// Возвращает идентификатор публикации.
    #[must_use]
    #[inline]
    pub fn publication_id(&self) -> PublicationId {
        self.inner.root_id()
    }

    /// Возвращает диапазон годов.
    #[must_use]
    #[inline]
    pub fn years(&self) -> YearSpan {
        self.inner.years()
    }

    /// Возвращает фильтр `i_ids`.
    #[must_use]
    #[inline]
    pub fn i_ids(&self) -> &[IndicatorId] {
        self.inner.i_ids()
    }

    /// Возвращает фильтр `m1_ids`.
    #[must_use]
    #[inline]
    pub fn m1_ids(&self) -> &[MeasureId] {
        self.inner.m1_ids()
    }

    /// Возвращает фильтр `m2_ids`.
    #[must_use]
    #[inline]
    pub fn m2_ids(&self) -> &[MeasureId] {
        self.inner.m2_ids()
    }
}

impl Serialize for DataExQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_multi_ids_query(serializer, "publicationId", &self.inner)
    }
}

/// Параметры для метода `/dataNew`.
#[derive(Debug, Clone)]
pub struct DataNewQuery {
    inner: MultiIdsQuery<CategoryId>,
}

impl DataNewQuery {
    /// Создаёт запрос с обязательными полями.
    #[must_use]
    #[inline]
    pub fn new(category_id: CategoryId, years: YearSpan) -> Self {
        Self {
            inner: MultiIdsQuery::new(category_id, years),
        }
    }

    /// Устанавливает массив `i_ids`.
    #[must_use]
    #[inline]
    pub fn with_i_ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = IndicatorId>,
    {
        self.inner = self.inner.with_i_ids(ids);
        self
    }

    /// Устанавливает массив `m1_ids`.
    #[must_use]
    #[inline]
    pub fn with_m1_ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = MeasureId>,
    {
        self.inner = self.inner.with_m1_ids(ids);
        self
    }

    /// Устанавливает массив `m2_ids`.
    #[must_use]
    #[inline]
    pub fn with_m2_ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = MeasureId>,
    {
        self.inner = self.inner.with_m2_ids(ids);
        self
    }

    /// Возвращает идентификатор категории.
    #[must_use]
    #[inline]
    pub fn category_id(&self) -> CategoryId {
        self.inner.root_id()
    }

    /// Возвращает диапазон годов.
    #[must_use]
    #[inline]
    pub fn years(&self) -> YearSpan {
        self.inner.years()
    }

    /// Возвращает фильтр `i_ids`.
    #[must_use]
    #[inline]
    pub fn i_ids(&self) -> &[IndicatorId] {
        self.inner.i_ids()
    }

    /// Возвращает фильтр `m1_ids`.
    #[must_use]
    #[inline]
    pub fn m1_ids(&self) -> &[MeasureId] {
        self.inner.m1_ids()
    }

    /// Возвращает фильтр `m2_ids`.
    #[must_use]
    #[inline]
    pub fn m2_ids(&self) -> &[MeasureId] {
        self.inner.m2_ids()
    }
}

impl Serialize for DataNewQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_multi_ids_query(serializer, "categoryId", &self.inner)
    }
}

#[derive(Debug, Clone)]
struct MultiIdsQuery<RootId> {
    root_id: RootId,
    years: YearsRangeQuery,
    i_ids: Vec<IndicatorId>,
    m1_ids: Vec<MeasureId>,
    m2_ids: Vec<MeasureId>,
}

impl<RootId: Copy> MultiIdsQuery<RootId> {
    #[inline]
    fn new(root_id: RootId, years: YearSpan) -> Self {
        Self {
            root_id,
            years: YearsRangeQuery::from_span(years),
            i_ids: Vec::new(),
            m1_ids: Vec::new(),
            m2_ids: Vec::new(),
        }
    }

    #[inline]
    fn with_i_ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = IndicatorId>,
    {
        self.i_ids = ids.into_iter().collect();
        self
    }

    #[inline]
    fn with_m1_ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = MeasureId>,
    {
        self.m1_ids = ids.into_iter().collect();
        self
    }

    #[inline]
    fn with_m2_ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = MeasureId>,
    {
        self.m2_ids = ids.into_iter().collect();
        self
    }

    #[inline]
    fn root_id(&self) -> RootId {
        self.root_id
    }

    #[inline]
    fn years(&self) -> YearSpan {
        self.years.span()
    }

    #[inline]
    fn i_ids(&self) -> &[IndicatorId] {
        &self.i_ids
    }

    #[inline]
    fn m1_ids(&self) -> &[MeasureId] {
        &self.m1_ids
    }

    #[inline]
    fn m2_ids(&self) -> &[MeasureId] {
        &self.m2_ids
    }
}

fn serialize_multi_ids_query<S, RootId>(
    serializer: S,
    root_key: &'static str,
    query: &MultiIdsQuery<RootId>,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    RootId: Copy + Serialize,
{
    let mut seq = serializer.serialize_seq(Some(
        3 + query.i_ids.len() + query.m1_ids.len() + query.m2_ids.len(),
    ))?;
    seq.serialize_element(&(root_key, query.root_id))?;
    seq.serialize_element(&("y1", query.years.start()))?;
    seq.serialize_element(&("y2", query.years.end()))?;
    serialize_repeated_ids(&mut seq, "i_ids", &query.i_ids)?;
    serialize_repeated_ids(&mut seq, "m1_ids", &query.m1_ids)?;
    serialize_repeated_ids(&mut seq, "m2_ids", &query.m2_ids)?;
    seq.end()
}

fn serialize_repeated_ids<S, Id>(seq: &mut S, key: &'static str, ids: &[Id]) -> Result<(), S::Error>
where
    S: SerializeSeq,
    Id: Copy + Serialize,
{
    for &id in ids {
        seq.serialize_element(&(key, id))?;
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, Serialize)]
pub(crate) struct PublicationIdQuery {
    #[serde(rename = "publicationId")]
    publication_id: PublicationId,
}

#[inline]
pub(crate) fn publication_id_query(publication_id: PublicationId) -> PublicationIdQuery {
    PublicationIdQuery { publication_id }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub(crate) struct DatasetIdQuery {
    #[serde(rename = "datasetId")]
    dataset_id: DatasetId,
}

#[inline]
pub(crate) fn dataset_id_query(dataset_id: DatasetId) -> DatasetIdQuery {
    DatasetIdQuery { dataset_id }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub(crate) struct YearsQuery {
    #[serde(rename = "datasetId")]
    dataset_id: DatasetId,
    #[serde(rename = "measureId", skip_serializing_if = "Option::is_none")]
    measure_id: Option<MeasureId>,
}

#[inline]
pub(crate) fn years_query(dataset_id: DatasetId, measure_id: Option<MeasureId>) -> YearsQuery {
    YearsQuery {
        dataset_id,
        measure_id,
    }
}

#[derive(Debug)]
pub(crate) struct YearsExQuery<'a> {
    publication_id: PublicationId,
    ids: &'a [DatasetId],
}

#[inline]
pub(crate) fn years_ex_query(publication_id: PublicationId, ids: &[DatasetId]) -> YearsExQuery<'_> {
    YearsExQuery {
        publication_id,
        ids,
    }
}

impl Serialize for YearsExQuery<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(1 + self.ids.len()))?;
        seq.serialize_element(&("publicationId", self.publication_id))?;
        serialize_repeated_ids(&mut seq, "ids", self.ids)?;
        seq.end()
    }
}
