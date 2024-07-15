mod astro;

use clap::Parser;
use chrono::prelude::*;
use std::error::Error;
use astro::{calculate_astrological_config, AstrologicalConfig};
use rust_bert::pipelines::text_generation::{TextGenerationModel, TextGenerationConfig};
use tokio::runtime::Runtime;

/// CLI tool to generate git commit messages based on astrological configurations
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Date for astrological calculation (YYYY-MM-DD)
    #[arg(short, long)]
    date: Option<String>,

    /// Time for astrological calculation (HH:MM)
    #[arg(short, long)]
    time: Option<String>,
}

fn generate_commit_message(prompt: &str) -> Result<String, Box<dyn Error>> {
    let rt = Runtime::new()?;
    let text = rt.block_on(async {
        let config = TextGenerationConfig {
            max_length: 200,
            num_beams: 5,
            repetition_penalty: 2.0,
            temperature: 1.6,
            ..Default::default()
        };
        let model = TextGenerationModel::new(config)?;

        // Generate text using the provided prompt
        let output = model.generate(&[prompt], None);
        Ok::<_, Box<dyn Error>>(output.join(" "))
    })?;

    Ok(text)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let now = Utc::now();
    let date_str = args.date.unwrap_or_else(|| now.format("%Y-%m-%d").to_string());
    let time_str = args.time.unwrap_or_else(|| now.format("%H:%M").to_string());

    let datetime_str = format!("{}T{}:00Z", date_str, time_str);
    let datetime_fixed_offset = DateTime::parse_from_rfc3339(&datetime_str)?;
    let datetime: DateTime<FixedOffset> = datetime_fixed_offset.into();

    println!("Using datetime: {}", datetime);

    let config: AstrologicalConfig = calculate_astrological_config(datetime);
    let astrological_prompt = format!(
        "The astrological configurations on {} are: Sun at {:.2}°, Moon at {:.2}°. Creative commit message in 20 words:",
        datetime, config.sun_position, config.moon_position
    );

    let commit_message = generate_commit_message(&astrological_prompt)?;
    println!("Suggested commit message: {}", commit_message);

    Ok(())
}
