// 1BRC - my take on the 1 Billion Row Challenge
// Copyright (C) 2024  Charles German <5donuts@pm.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::path::{Path, PathBuf};
use std::time::Duration;

use clap::Parser;
use serde::Serialize;

use helpers::ChallengeRunner;

mod helpers;
mod runners;

#[derive(Debug, Default, Clone, Copy, Serialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
enum Runner {
    /// Iterate through the input line-by-line
    Baseline,

    /// Read the file into memory in a few large chunks instead of many smaller I/O ops
    #[default]
    Chunks,
}

#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    about,
    long_about = r#"My take on the 1 Billion Row Challenge

Copyright (C) 2024 Charles German <5donuts@pm.me>
This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY.
See the GNU General Public License for more details. You should have received a copy of the
GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>."#
)]
struct Args {
    /// The runner to use to solve the challenge
    #[clap(short, long, default_value_t, value_enum)]
    runner: Runner,

    /// Path to the file containing the challenge input
    #[clap(value_parser)]
    input: PathBuf,

    /// Benchmark the selected runner
    ///
    /// The runner is invoked five times sequentially with the fastest and slowest times discarded.
    /// Then, the mean & standard deviation of runtimes is displayed.
    #[clap(short, long, action)]
    bench: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let runner = args.runner;
    let input = &args.input;

    if args.bench {
        benchmark(runner, input)
    } else {
        run(runner, input, true).and_then(|_| Ok(()))
    }
}

/// Run the selected [`Runner`] against the provided input.
/// If `print_output = true`, print the result to stdout.
/// Return the duration it took to compute the result.
fn run(
    runner: Runner,
    input: &Path,
    print_output: bool,
) -> Result<Duration, Box<dyn std::error::Error>> {
    use Runner::*;
    let (station_info, duration) = match runner {
        Baseline => runners::Baseline::run(input),
        Chunks => runners::Chunks::run(input),
    }?;

    if print_output {
        // Display the results with wrapping '{ ... }' and ',' between each entry, but
        // not following the last entry.
        print!("{{");
        for i in 0..(station_info.len() - 1) {
            print!("{}", station_info[i]);
            print!(", ");
        }
        println!("{}}}\n", station_info.iter().last().unwrap());
        println!("Solved in {}", fmt_duration(&duration));
    }

    Ok(duration)
}

/// Benchmark the selected [`Runner`] using the provided input
///
/// The runner is invoked five times. The fastest and slowest times are discarded.
/// Then, the mean and standard deviation of runs is calculated.
///
/// All times as well as the benchmark result are shown to the user.
fn benchmark(runner: Runner, input: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Collect the run results
    let durations: Result<Vec<Duration>, _> = (1..=5)
        .into_iter()
        .map(|i| {
            run(runner, input, false).and_then(|duration| {
                println!("Run {i}: {}", fmt_duration(&duration));
                Ok(duration)
            })
        })
        .collect();
    let mut durations = durations?;
    durations.sort();

    // Drop the highest & lowest runs
    let durations = &durations[1..4];

    // Compute some basic stats
    let duration_millis: Vec<_> = durations
        .iter()
        .map(|d| d.as_secs() * 1000 + d.subsec_millis() as u64)
        .collect();
    let mean =
        duration_millis.iter().map(|&m| m as f64).sum::<f64>() / duration_millis.len() as f64;
    let variance = duration_millis
        .iter()
        .map(|&m| (m as f64 - mean).powf(2.0))
        .sum::<f64>()
        / duration_millis.len() as f64;
    let std_dev = variance.sqrt();

    // Convert the stats back to durations & print the result
    let mean = Duration::from_millis(mean.ceil() as u64);
    let std_dev = Duration::from_millis(std_dev.ceil() as u64);

    println!(
        "\nMean: {} ± {}",
        fmt_duration(&mean),
        fmt_duration(&std_dev)
    );
    Ok(())
}

/// Helper function to format a [`Duration`] with a nice seconds/ms structure
fn fmt_duration(duration: &Duration) -> String {
    // Display the time it took to compute the results
    let seconds = duration.as_secs();
    let millis = duration.subsec_millis();
    format!("{seconds}s {millis:0>3}ms")
}

#[cfg(test)]
mod tests {
    use crate::helpers::*;
    use once_cell::sync::Lazy;

    static TEST_DATA: &'static str = r#"Glens Falls;-47.5
Shimanto;30.3
Zverevo;98.1
Shimanto;74.9
Zverevo;87.6
Aïn el Mediour;47.6
Paidiipalli;91.1
Shimanto;27.5
Aïn el Mediour;5.7
Shimanto;20.9
Glens Falls;6.6
"#;

    static EXPECTED_RESULT: Lazy<Vec<StationInfo>> = Lazy::new(|| {
        vec![
            StationInfo::new(String::from("Aïn el Mediour"), 5.7, 47.6, 26.65),
            StationInfo::new(String::from("Glens Falls"), -47.5, 6.6, -20.45),
            StationInfo::new(String::from("Paidiipalli"), 91.1, 91.1, 91.1),
            StationInfo::new(String::from("Shimanto"), 20.9, 74.9, 38.4),
            StationInfo::new(String::from("Zverevo"), 87.6, 87.6, 87.6),
        ]
    });
}
