use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use thiserror::Error;

const BODY_PREVIEW_LIMIT: usize = 512;

/// Ошибки клиента API ЦБ РФ.
#[derive(Debug, Error)]
pub enum CbrError {
    /// Ошибка транспорта (сетевые сбои, таймауты, ошибки TLS и т.п.).
    #[error("transport error: {0}")]
    Transport(reqwest::Error),
    /// Ошибка построения HTTP-клиента.
    #[error("client build error: {0}")]
    Build(reqwest::Error),
    /// API вернул HTTP-статус вне диапазона 2xx.
    #[error("api returned status {status} (body {body_size} bytes): {body_preview}")]
    Status {
        status: StatusCode,
        body_preview: String,
        body_size: usize,
    },
    /// Тело ответа не удалось десериализовать в ожидаемую модель.
    #[error(
        "failed to deserialize response (body {body_size} bytes): {source}; preview: {body_preview}"
    )]
    Deserialize {
        source: serde_json::Error,
        body_preview: String,
        body_size: usize,
    },
    /// Legacy-ответ сервиса в формате `{Error:true}`.
    #[error("api returned legacy error payload ({payload_size} bytes): {payload_preview}")]
    LegacyErrorResponse {
        payload_preview: String,
        payload_size: usize,
    },
}

impl CbrError {
    pub(crate) fn transport(source: reqwest::Error) -> Self {
        Self::Transport(source)
    }

    pub(crate) fn build(source: reqwest::Error) -> Self {
        Self::Build(source)
    }

    pub(crate) fn status(status: StatusCode, body: &[u8]) -> Self {
        let (body_preview, body_size) = summarize_body(body);
        Self::Status {
            status,
            body_preview,
            body_size,
        }
    }

    pub(crate) fn deserialize(source: serde_json::Error, body: &[u8]) -> Self {
        let (body_preview, body_size) = summarize_body(body);
        Self::Deserialize {
            source,
            body_preview,
            body_size,
        }
    }

    pub(crate) fn legacy_error_payload(body: &[u8]) -> Self {
        let (payload_preview, payload_size) = summarize_body(body);
        Self::LegacyErrorResponse {
            payload_preview,
            payload_size,
        }
    }
}

pub(crate) fn parse_json_body<T>(status: StatusCode, body: &[u8]) -> Result<T, CbrError>
where
    T: DeserializeOwned,
{
    // Иногда API возвращает невалидный JSON вида `{Error:true}`.
    // Обрабатываем его отдельно до проверки статуса/десериализации.
    if is_legacy_error_payload(body) {
        return Err(CbrError::legacy_error_payload(body));
    }

    if !status.is_success() {
        return Err(CbrError::status(status, body));
    }

    serde_json::from_slice(body).map_err(|source| CbrError::deserialize(source, body))
}

fn summarize_body(body: &[u8]) -> (String, usize) {
    let total_size = body.len();
    let preview_size = total_size.min(BODY_PREVIEW_LIMIT);
    let mut preview = String::from_utf8_lossy(&body[..preview_size]).into_owned();

    if total_size > BODY_PREVIEW_LIMIT {
        preview.push_str("...<truncated>");
    }

    (preview, total_size)
}

fn is_legacy_error_payload(body: &[u8]) -> bool {
    let Ok(text) = std::str::from_utf8(body) else {
        return false;
    };
    let trimmed = text.trim();
    let Some(inner) = trimmed
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
    else {
        return false;
    };

    let mut parts = inner.trim().split(':');
    let Some(raw_key) = parts.next() else {
        return false;
    };
    let Some(raw_value) = parts.next() else {
        return false;
    };
    if parts.next().is_some() {
        return false;
    }

    let key = raw_key.trim().trim_matches(|c| c == '"' || c == '\'');
    let value = raw_value
        .trim()
        .trim_end_matches(',')
        .trim()
        .trim_matches(|c| c == '"' || c == '\'');

    key.eq_ignore_ascii_case("error") && value.eq_ignore_ascii_case("true")
}
