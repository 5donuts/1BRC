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

use std::path::PathBuf;

use serde::Serialize;
use clap::Parser;

use helpers::ChallengeRunner;

mod naive;
mod helpers;

#[derive(Debug, Default, Clone, Serialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
enum Runner {
    /// Naive approach to the challenge
    ///
    /// Iterate through the input using a single thread to build min/max/avg data
    /// for each station, without loading the entire input into memory.
    #[default]
    Naive,
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
    /// The name of the runner to use to solve the challenge.
    #[clap(short, long, default_value_t, value_enum)]
    runner: Runner,

    /// Path to the file containing the challenge input
    #[clap(value_parser)]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Process arguments
    let args = Args::parse();
    let input = &args.input;
    let runner = args.runner;

    // Do the thing
    use Runner::*;
    let (station_info, duration) = match runner {
        Naive => naive::Runner::run(input),
    }?;

    // Display the result
    print!("{{");
    for i in 0..station_info.len() {
        print!("{}", station_info[i]);
        if station_info.len() - 1 > 0 {
            print!(", ");
        }
    }
    println!("}}");

    // Display the time it took to compute the results
    let seconds = duration.as_secs();
    let millis = duration.subsec_millis();
    let micros = duration.subsec_micros() - (millis * 1000);
    println!("Solved in {seconds}s {millis:0>3}ms {micros:0>3}µs");

    Ok(())
}
