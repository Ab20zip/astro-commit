use chrono::{DateTime, FixedOffset};

pub struct AstrologicalConfig {
    pub sun_position: f64,
    pub moon_position: f64,
}

pub fn calculate_astrological_config(datetime: DateTime<FixedOffset>) -> AstrologicalConfig {
    let timestamp = datetime.timestamp();
    let sun_position = (timestamp % 3600) as f64 / 10.0;
    let moon_position = (timestamp % 1800) as f64 / 20.0;

    AstrologicalConfig {
        sun_position,
        moon_position,
    }
}
