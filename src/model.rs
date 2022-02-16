use super::error::Error;
use chrono::{Date, DateTime, TimeZone, Utc};
use serde_derive::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

pub fn read_flagged_file<P: AsRef<Path>>(path: P) -> Result<HashMap<u64, usize>, Error> {
    let mut data = csv::Reader::from_reader(File::open(path)?);

    data.records()
        .enumerate()
        .map(|(i, result)| {
            let csv_record = result?;
            let id = csv_record[0]
                .parse::<u64>()
                .map_err(|_| Error::InvalidFlaggedRecord(csv_record.clone()))?;

            Ok((id, i + 1))
        })
        .collect::<Result<HashMap<_, _>, Error>>()
}

#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct SuspensionRecord {
    pub timestamp: DateTime<Utc>,
    pub reversal: Option<DateTime<Utc>>,
    pub user_id: u64,
    pub created_at: DateTime<Utc>,
    pub screen_name: String,
    pub verified: bool,
    pub protected: bool,
    pub followers_count: usize,
    pub profile_image_url: String,
    pub withheld_in_countries: Vec<String>,
}

impl SuspensionRecord {
    fn read_file_raw<P: AsRef<Path>>(
        path: P,
    ) -> Result<HashMap<Date<Utc>, Vec<Option<Self>>>, Error> {
        let mut data = csv::Reader::from_reader(File::open(path)?);
        let mut by_date: HashMap<Date<Utc>, Vec<Option<Self>>> = HashMap::new();

        for result in data.records() {
            let csv_record = result?;
            let (record, date) = if csv_record[3].is_empty() {
                (
                    None,
                    Utc.timestamp(csv_record[0].parse::<i64>().unwrap(), 0)
                        .date(),
                )
            } else {
                let record = Self::try_from(csv_record)?;
                let date = record.timestamp.date();
                (Some(record), date)
            };

            let records = by_date.entry(date).or_default();
            records.push(record);
        }

        Ok(by_date)
    }

    pub fn read_file<P: AsRef<Path>>(
        path: P,
    ) -> Result<HashMap<Date<Utc>, (Vec<Self>, usize)>, Error> {
        let by_date = Self::read_file_raw(path)?;

        let results = by_date
            .into_iter()
            .map(|(date, records)| {
                let unknown_count = records
                    .iter()
                    .filter(|maybe_record| maybe_record.is_none())
                    .count();

                let new_records = records.into_iter().flatten().collect::<Vec<_>>();

                (date, (new_records, unknown_count))
            })
            .collect::<HashMap<_, _>>();

        Ok(results)
    }
}

impl TryFrom<csv::StringRecord> for SuspensionRecord {
    type Error = Error;

    fn try_from(value: csv::StringRecord) -> Result<Self, Self::Error> {
        if value.len() == 10 {
            let (
                (((((timestamp, reversal), user_id), created_at), verified), protected),
                followers_count,
            ) = value[0]
                .parse::<i64>()
                .map(|timestamp_s| Utc.timestamp(timestamp_s, 0))
                .ok()
                .zip(if value[1].is_empty() {
                    Some(None)
                } else {
                    value[1]
                        .parse::<i64>()
                        .map(|timestamp_s| Some(Utc.timestamp(timestamp_s, 0)))
                        .ok()
                })
                .zip(value[2].parse::<u64>().ok())
                .zip(
                    value[3]
                        .parse::<i64>()
                        .map(|timestamp_s| Utc.timestamp(timestamp_s, 0))
                        .ok(),
                )
                .zip(value[5].parse::<bool>().ok())
                .zip(value[6].parse::<bool>().ok())
                .zip(value[7].parse::<usize>().ok())
                .ok_or_else(|| Self::Error::InvalidScreenNamesRecord(value.clone()))?;

            Ok(Self {
                timestamp,
                reversal,
                user_id,
                created_at,
                screen_name: value[4].to_string(),
                verified,
                protected,
                followers_count,
                profile_image_url: value[8].to_string(),
                withheld_in_countries: value[9]
                    .split(';')
                    .filter_map(|code| {
                        if code.is_empty() {
                            None
                        } else {
                            Some(code.to_string())
                        }
                    })
                    .collect(),
            })
        } else {
            Err(Self::Error::InvalidSuspensionsRecord(value))
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ScreenNameRecord {
    pub timestamp: DateTime<Utc>,
    pub user_id: u64,
    pub verified: bool,
    pub protected: bool,
    pub followers_count: usize,
    pub previous_screen_name: String,
    pub new_screen_name: String,
    pub profile_image_url: String,
}

impl ScreenNameRecord {
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<HashMap<Date<Utc>, Vec<Self>>, Error> {
        let mut data = csv::Reader::from_reader(File::open(path)?);
        let mut by_date: HashMap<Date<Utc>, Vec<Self>> = HashMap::new();

        for result in data.records() {
            let record = Self::try_from(result?)?;
            let date = record.timestamp.date();

            let records = by_date.entry(date).or_default();
            records.push(record);
        }

        Ok(by_date)
    }
}

impl TryFrom<csv::StringRecord> for ScreenNameRecord {
    type Error = Error;

    fn try_from(value: csv::StringRecord) -> Result<Self, Self::Error> {
        if value.len() == 8 {
            let ((((timestamp, user_id), verified), protected), followers_count) = value[0]
                .parse::<i64>()
                .map(|timestamp_s| Utc.timestamp(timestamp_s, 0))
                .ok()
                .zip(value[1].parse::<u64>().ok())
                .zip(value[2].parse::<bool>().ok())
                .zip(value[3].parse::<bool>().ok())
                .zip(value[4].parse::<usize>().ok())
                .ok_or_else(|| Self::Error::InvalidScreenNamesRecord(value.clone()))?;

            Ok(Self {
                timestamp,
                user_id,
                verified,
                protected,
                followers_count,
                previous_screen_name: value[5].to_string(),
                new_screen_name: value[6].to_string(),
                profile_image_url: value[7].to_string(),
            })
        } else {
            Err(Self::Error::InvalidScreenNamesRecord(value))
        }
    }
}
