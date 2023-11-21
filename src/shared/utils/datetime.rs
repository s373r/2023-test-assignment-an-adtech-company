use chrono::NaiveDateTime;

pub fn datetime_now() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}
