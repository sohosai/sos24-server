use anyhow::Context;
use csv::Writer;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CsvSerializationError {
    #[error(transparent)]
    FailedToSerialize(#[from] anyhow::Error),
}

pub fn serialize_to_csv<S: Serialize>(records: Vec<S>) -> Result<String, CsvSerializationError> {
    // 文字化けを防ぐためにUTF-8 BOMを付けておく
    let mut wrt = Writer::from_writer(vec![0xEF, 0xBB, 0xBF]);
    for record in records {
        wrt.serialize(record).context("Failed to serialize")?;
    }

    let csv = wrt.into_inner().context("Failed to write csv")?;
    let data = String::from_utf8(csv).context("Failed to convert csv to string")?;
    Ok(data)
}
