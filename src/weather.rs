use openweathermap::blocking::weather;

const celsius_symbol: &str = "°C";

pub struct Weather<'a> {
    api_key: &'a str,
}

impl<'a> Weather<'a> {
    pub fn new(api_key: &'a str) -> Self {
        Self { api_key }
    }

    pub fn get_temp(
        &self,
        location: &str,
    ) -> std::result::Result<f64, String> {
        match &weather(location, "metric", "en", self.api_key) {
            Ok(current) => Ok(current.main.temp),
            Err(e) => Err(format!("cannot get temperature: {}", e)),
        }
    }

    pub fn get_temp_formatted(
        &self,
        location: &str,
    ) -> std::result::Result<String, String> {
        let temp = self.get_temp(location)?;
        Ok(format!("{temp}{celsius_symbol}"))
    }

    pub fn get_today_stats(
        &self,
        location: &str,
    ) -> std::result::Result<String, String> {
        let mut lines = Vec::new();
        match &weather(location, "metric", "en", self.api_key) {
            Ok(current) => {
                lines.push(format!(
                    "current temp: {}{}",
                    current.main.temp, celsius_symbol
                ));
                lines.push(format!(
                    "max temp: {}{}",
                    current.main.temp_max, celsius_symbol
                ));
                lines.push(format!(
                    "min temp: {}{}",
                    current.main.temp_min, celsius_symbol
                ));
                lines.push(format!(
                    "feels like: {}{}",
                    current.main.feels_like, celsius_symbol
                ));
                lines.push(format!(
                    "humidity: {}{}",
                    current.main.humidity, celsius_symbol
                ));
                Ok(lines.join("\n"))
            },
            Err(e) => Err(format!("cannot get temperature: {}", e)),
        }
    }
}

pub fn get_current_temperature(
    location: &str,
    api_key: &str,
) -> std::result::Result<f64, String> {
    match &weather(location, "metric", "en", api_key) {
        Ok(current) => Ok(current.main.temp),
        Err(e) => Err(format!("cannot get temperature: {}", e)),
    }
}

pub fn get_temperature_formatted(
    location: &str,
    api_key: &str,
) -> std::result::Result<String, String> {
    let temp = get_current_temperature(location, api_key)?;
    Ok(format!("{temp}{celsius_symbol}"))
}
