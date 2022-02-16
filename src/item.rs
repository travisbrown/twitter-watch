use super::model::{ScreenNameRecord, SuspensionRecord};
use chrono::NaiveDate;
use serde_derive::Serialize;
use std::cmp::Ordering;

#[derive(Clone, Default, PartialEq, Serialize)]
pub struct Stats {
    pub total_suspensions_count: usize,
    pub total_reversals_count: usize,
    pub total_screen_name_changes_count: usize,
    pub total_verified_suspended_count: usize,
    pub total_protected_suspended_count: usize,
    pub total_withheld_suspended_count: usize,
    pub other: Vec<(String, ((f64, f64, f64), f64))>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Serialize)]
pub struct DateStats {
    pub date: NaiveDate,
    pub total_suspensions_count: usize,
    pub tracked_suspensions_count: usize,
    pub total_screen_name_changes_count: usize,
    pub tracked_screen_name_changes_count: usize,
}

#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct SuspensionItem<'a> {
    pub record: &'a SuspensionRecord,
    pub image_url: String,
    pub ranking: Option<usize>,
    pub other_screen_name_count: usize,
}

impl<'a> Ord for SuspensionItem<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ranking
            .cmp(&other.ranking)
            .then_with(|| {
                self.record
                    .followers_count
                    .cmp(&other.record.followers_count)
                    .reverse()
            })
            .then_with(|| self.record.user_id.cmp(&other.record.user_id))
    }
}

impl<'a> PartialOrd for SuspensionItem<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ScreenNameItem<'a> {
    pub record: &'a ScreenNameRecord,
    pub image_url: String,
    pub ranking: Option<usize>,
    pub other_screen_name_count: usize,
}

impl<'a> Ord for ScreenNameItem<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ranking
            .cmp(&other.ranking)
            .then_with(|| {
                self.record
                    .followers_count
                    .cmp(&other.record.followers_count)
                    .reverse()
            })
            .then_with(|| self.record.user_id.cmp(&other.record.user_id))
    }
}

impl<'a> PartialOrd for ScreenNameItem<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
