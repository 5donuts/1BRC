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
use std::fs::File;
use std::io::{self, BufRead, Cursor, Read, Seek, SeekFrom};
use std::path::Path;
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
    fn run(input: &Path) -> ChallengeResult {
        let start = Instant::now();

        // Compute the approximate size of each chunk based on the size of the file
        let file_bytes = File::open(input)?.metadata()?.len();
        let bytes_per_step = (file_bytes as f32 / NUM_CHUNKS as f32).floor() as usize;

        // Read the file in a few large chunks, process each chunk using the same technique as in
        // src/baseline.rs
        let mut map: HashMap<String, StationData> = HashMap::new();
        let mut f = File::open(input)?;
        for _ in 0..NUM_CHUNKS {
            // Read the next chunk into memory
            let mut buf = vec![0; bytes_per_step];
            let buf = match f.read_exact(&mut buf) {
                // Handle the case that there are enough bytes remaining in the file to fill the
                // buffer
                Ok(_) => {
                    // Find the last newline in the chunk to handle the case that this chunk
                    // boundary splits a line
                    let end_idx = last_chunk_newline_idx(&buf);

                    // Move the cursor just past the last newline in the chunk so the next chunk
                    // starts at the beginning of a line rather than partway through one.
                    f.seek(SeekFrom::Current(end_idx as i64))?;

                    // Trim the buffer to not include the partial line
                    &buf[..end_idx]
                }
                Err(e) => match e.kind() {
                    // Handle the case that there are fewer bytes remaining in the file than can be
                    // held by the buffer
                    io::ErrorKind::UnexpectedEof => {
                        // Read the remaining bytes in the file then trim the slice to only the
                        // bytes that were just read
                        let bytes_read = f.read_to_end(&mut buf)?;
                        Ok(&buf[..bytes_read])
                    }
                    // Bubble up any other error
                    _ => Err(e),
                }?,
            };

            // Process the lines in the chunk using the same technique as src/baseline.rs
            for line in Cursor::new(buf).lines() {
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

/// Return the index of the last '\n' in the chunk
fn last_chunk_newline_idx(chunk: &[u8]) -> usize {
    chunk
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(idx, c)| if *c == b'\n' { Some(idx) } else { None })
        .next()
        .expect("Unable to find any '\\n' in chunk")
}
