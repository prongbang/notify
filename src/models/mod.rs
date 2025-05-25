use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct DateTime {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

#[derive(Debug, Clone)]
pub struct BuddhaDate {
    pub today: DateTime,
    pub tomorrow: DateTime,
    pub year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuddhaDay {
    pub description: String,
    pub found: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buddha {
    pub today: BuddhaDay,
    pub tomorrow: BuddhaDay,
}

impl Buddha {
    pub fn new() -> Self {
        Self {
            today: BuddhaDay {
                description: String::new(),
                found: false,
            },
            tomorrow: BuddhaDay {
                description: String::new(),
                found: false,
            },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub key: String,
}

impl BuddhaDate {
    pub fn from_now() -> Self {
        let today = Local::now().date_naive();
        let tomorrow = today.succ_opt().unwrap_or(today);

        Self {
            year: today.year() + 543, // Convert to Buddhist Era
            today: DateTime {
                day: today.day() as i32,
                month: today.month() as i32,
                year: today.year(),
            },
            tomorrow: DateTime {
                day: tomorrow.day() as i32,
                month: tomorrow.month() as i32,
                year: tomorrow.year(),
            },
        }
    }
}

impl DateTime {
    pub fn format_date(&self) -> String {
        format!(
            "{}{}{}",
            self.year,
            format_number(self.month),
            format_number(self.day)
        )
    }
}

fn format_number(n: i32) -> String {
    if n > 9 {
        n.to_string()
    } else {
        format!("0{}", n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date() {
        let dt = DateTime {
            day: 5,
            month: 9,
            year: 2024,
        };
        assert_eq!(dt.format_date(), "20240905");

        let dt = DateTime {
            day: 15,
            month: 12,
            year: 2024,
        };
        assert_eq!(dt.format_date(), "20241215");
    }

    #[test]
    fn test_buddha_new() {
        let buddha = Buddha::new();
        assert!(!buddha.today.found);
        assert!(!buddha.tomorrow.found);
        assert!(buddha.today.description.is_empty());
        assert!(buddha.tomorrow.description.is_empty());
    }
}
