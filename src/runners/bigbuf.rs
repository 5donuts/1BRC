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

use std::collections::HashMap;
use std::io::{BufRead, BufReader, SeekFrom};
use std::time::Instant;

use crate::helpers::*;

pub struct Runner;

/// Number of chunks into which to split the file when reading it into memory
const NUM_CHUNKS: usize = 3;

struct StationData {
    min: f32,
    max: f32,

    // Rather than compute a new average at each step, just keep a rolling sum
    // of all the measurements and calculate the average at the end.
    sum: f32,
    cnt: u32,
}

impl StationData {
    /// Instantiate a new record of measurements for a station
    fn new(measurement: f32) -> Self {
        Self {
            min: measurement,
            max: measurement,
            sum: measurement,
            cnt: 1,
        }
    }

    /// Record an additional measurement for this station
    fn push(&mut self, measurement: f32) {
        if measurement < self.min {
            self.min = measurement;
        } else if measurement > self.max {
            self.max = measurement;
        }

        self.sum += measurement;
        self.cnt += 1;
    }

    fn avg(&self) -> f32 {
        self.sum / self.cnt as f32
    }
}

impl ChallengeRunner for Runner {
    fn run<R>(input: R) -> ChallengeResult
    where
        R: std::io::Read + std::io::Seek,
    {
        let start = Instant::now();

        // Estimate the number of bytes in each chunk based on the number of bytes in the stream
        let mut input = input;
        let stream_bytes = {
            let end_offset = input.seek(SeekFrom::End(0))?;
            input.rewind()?;
            end_offset
        };
        let bytes_per_step = (stream_bytes as f32 / NUM_CHUNKS as f32).floor() as usize;

        // Configure our BufReader to use a larger buffer so we make fewer I/O operations while
        // reading the file. Otherwise, this approach is identical to the one in src/baseline.rs
        let mut map: HashMap<String, StationData> = HashMap::new();
        for line in BufReader::with_capacity(bytes_per_step, input).lines() {
            let line = line?;
            let mut parts = line.split(';');
            let station = parts.next().unwrap();
            let measurement = parts.next().unwrap().parse::<f32>()?;

            if let Some(station_data) = map.get_mut(station) {
                station_data.push(measurement);
            } else {
                let station_data = StationData::new(measurement);
                map.insert(station.to_owned(), station_data);
            }
        }

        // Build the alphabetically-sorted list of stations
        let mut stations: Vec<StationInfo> = map
            .into_iter()
            .map(|(key, val)| StationInfo::new(key, val.min, val.max, val.avg()))
            .collect();
        stations.sort_unstable();

        // Compute the time it took to generate the list of sorted stations
        let stop = Instant::now();
        let duration = stop.duration_since(start);

        Ok((stations, duration))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runners::tests::*;
    use std::{io, error};

    #[test]
    fn correctness() -> Result<(), Box<dyn error::Error>> {
        let input = io::Cursor::new(TEST_DATA.as_bytes());

        let (actual, _) = Runner::run(input)?;
        assert_eq!(actual, *EXPECTED_RESULT, "actual != expected for chunks runner");

        Ok(())
    }
}
