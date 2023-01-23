use eu4save::PdsDate;

pub mod hasher;
pub mod parser;
pub mod storage;

pub fn eu4_days_to_date(days: i32) -> String {
    eu4save::eu4_start_date()
        .add_days(days)
        .iso_8601()
        .to_string()
}
