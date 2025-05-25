use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::OnceCell;
use tracing::info;

use crate::{
    config::Config,
    error::{AppError, AppResult},
    models::{Buddha, BuddhaDate, BuddhaDay},
};

type BuddhaCache = Arc<Mutex<HashMap<i32, HashMap<String, Vec<String>>>>>;
static BUDDHA_CACHE: OnceCell<BuddhaCache> = OnceCell::const_new();

#[derive(Clone)]
pub struct BuddhaServiceImpl {
    client: reqwest::Client,
    config: Config,
}

impl BuddhaServiceImpl {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    pub async fn get_buddha(&self, date: BuddhaDate) -> AppResult<Buddha> {
        let data = self.get_buddha_data(date.year).await?;
        let mut buddha = Buddha::new();

        if let Some(year_data) = data.get(&date.year) {
            let today_date = date.today.format_date();
            let tomorrow_date = date.tomorrow.format_date();

            if let Some(today_data) = year_data.get(&today_date) {
                if !today_data.is_empty() {
                    buddha.today = BuddhaDay {
                        description: format!("วันนี้ {}", today_data[0]),
                        found: true,
                    };
                }
            }

            if let Some(tomorrow_data) = year_data.get(&tomorrow_date) {
                if !tomorrow_data.is_empty() {
                    buddha.tomorrow = BuddhaDay {
                        description: format!("พรุ่งนี้ {}", tomorrow_data[0]),
                        found: true,
                    };
                }
            }
        }

        Ok(buddha)
    }

    async fn get_buddha_data(
        &self,
        year: i32,
    ) -> AppResult<HashMap<i32, HashMap<String, Vec<String>>>> {
        let cache = BUDDHA_CACHE
            .get_or_init(|| async { Arc::new(Mutex::new(HashMap::new())) })
            .await;

        {
            let cache_guard = cache
                .lock()
                .map_err(|e| AppError::CacheError(e.to_string()))?;
            if let Some(data) = cache_guard.get(&year) {
                if !data.is_empty() {
                    info!("Cache hit for year: {}", year);
                    return Ok(HashMap::from([(year, data.clone())]));
                }
            }
        }

        let mut buddha_record = HashMap::new();
        let url = format!("{}?{}.csv", self.config.buddha_endpoint, year);

        info!("Fetching Buddha data for year: {}", year);
        let response = self.client.get(&url).send().await?;
        let csv_data = response.text().await?;
        let mut rdr = csv::Reader::from_reader(csv_data.as_bytes());

        for result in rdr.records() {
            let record = result?;
            if record.len() >= 2 {
                let description = &record[0];
                let date = &record[1];

                if description.contains("วันพระ") || description.contains("15 ค่ำ")
                {
                    buddha_record.insert(
                        date.to_string(),
                        record.iter().map(|s| s.to_string()).collect(),
                    );
                }
            }
        }

        {
            let mut cache_guard = cache
                .lock()
                .map_err(|e| AppError::CacheError(e.to_string()))?;
            cache_guard.insert(year, buddha_record);
            Ok(HashMap::from([(year, cache_guard[&year].clone())]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_buddha_service() {
        let config = Config::new().unwrap();
        let service = BuddhaServiceImpl::new(config);

        let date = BuddhaDate::from_now();
        let result = service.get_buddha(date).await;

        assert!(result.is_ok());
    }
}
