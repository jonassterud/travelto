use anyhow::Result;

/// Format duration as `"{}h {}m"`.
/// 
/// # Arguments
/// 
/// * `seconds` - amount of seconds.
pub fn format_duration(seconds: u64) -> String {
    format!("{}h {}m", (seconds / 60) / 60, (seconds / 60) % 60)
}

/// Format date as `"dd/mm, HH:MM"`.
/// 
/// # Arguments
/// 
/// * `date` - ISO 8601 date.
pub fn format_date(date: &str) -> Result<String> {
    Ok(date
        .parse::<chrono::DateTime<chrono::Utc>>()?
        .format("%d/%m, %R")
        .to_string())
}