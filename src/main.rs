use anyhow::{Context, Result, bail};
use clap::Parser;
use csv::StringRecord;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

/// CLI arguments -------------------------------------------------------------
#[derive(Parser, Debug)]
#[command(author, version, about = "CSV summary‑statistics tool")]
struct Cli {
    /// Path of the CSV file to process
    input: PathBuf,
}

/// Per‑column running stats --------------------------------------------------
#[derive(Debug, Default, Clone)]
struct Stats {
    count: usize,
    min: f64,
    max: f64,
    running: f64, // running sum for average
}

impl Stats {
    fn update(&mut self, value: f64) {
        if self.count == 0 {
            self.min = value;
            self.max = value;
        } else {
            self.min = self.min.min(value);
            self.max = self.max.max(value);
        }
        self.running += value;
        self.count += 1;
    }
    fn average(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.running / self.count as f64
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.input.exists() {
        bail!("File `{}` does not exist", cli.input.display());
    }

    let file =
        File::open(&cli.input).with_context(|| format!("Cannot open `{}`", cli.input.display()))?;
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true) // tolerate missing fields
        .from_reader(file);

    let headers = rdr
        .headers()
        .context("CSV is empty or missing header row")?
        .clone();

    // Map header_idx -> Stats
    let mut stats: HashMap<usize, Stats> = HashMap::new();

    for result in rdr.records() {
        let record = result.context("Failed to read a CSV record")?;
        process_record(&headers, &record, &mut stats);
    }

    if stats.is_empty() {
        println!("No numeric columns detected.");
        return Ok(());
    }

    println!("Summary statistics for numeric columns:");
    for (idx, header) in headers.iter().enumerate() {
        if let Some(s) = stats.get(&idx) {
            println!(
                "{:<20} -> count: {:>3}, min: {:>10.2}, max: {:>10.2}, average: {:>10.2}",
                header,
                s.count,
                s.min,
                s.max,
                s.average()
            );
        }
    }
    Ok(())
}

/// Parses all fields in `record`; updates `stats` whenever a field parses to f64.
fn process_record(
    headers: &StringRecord,
    record: &StringRecord,
    stats: &mut HashMap<usize, Stats>,
) {
    for (idx, field) in record.iter().enumerate() {
        if field.trim().is_empty() {
            continue; // missing value -> skip
        }
        if let Ok(val) = field.parse::<f64>() {
            stats.entry(idx).or_default().update(val);
        } else {
            // Non‑numeric value -> ensures column is treated as non‑numeric
            stats.remove(&idx);
        }
    }
}
