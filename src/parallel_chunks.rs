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

use rayon::prelude::*;

use crate::helpers::*;

pub struct Runner;

/// Number of chunks into which to split the file when reading it into memory
const NUM_CHUNKS: usize = 4;

/// Number of chunks to read simultaneously (ideally, a divisor of `NUM_CHUNKS`)
const PARALLEL_CHUNKS: usize = 2;

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

impl std::ops::Add for StationData {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let min = if self.min < other.min { self.min } else { other.min };
        let max = if self.max > other.max { self.max } else { other.max };
        let sum = self.sum + other.sum;
        let cnt = self.cnt + other.cnt;

        Self {
            min, max, sum, cnt
        }
    }
}

impl ChallengeRunner for Runner {
    fn run(input: &Path) -> ChallengeResult {
        let start = Instant::now();

        // Compute the exact chunk boundaries
        let start_indices = find_chunk_boundaries(input)?;
        let mut boundaries = Vec::new();
        for i in 0..start_indices.len() {
            let start = start_indices[i];
            let end = if i < start_indices.len() - 1 {
                start_indices[i + 1]
            } else {
                // If this is the last chunk in the file, we want to read to the end
                std::fs::metadata(input)?.len() as usize
            };

            boundaries.push((start, end));
        }

        // Spawn threads that will read the file & process the chunks in parallel
        // TODO: do some actual error handling
        rayon::ThreadPoolBuilder::new().num_threads(PARALLEL_CHUNKS).build_global()?;
        let mut maps: Vec<_> = boundaries.par_iter().map(|(start, end): &(usize, usize)| {
            // Load the chunk
            let mut f = File::open(input).unwrap();

            let mut buf = vec![0; end - start];
            f.seek(SeekFrom::Current(*start as i64)).unwrap();
            f.read_exact(&mut buf).unwrap();

            // Process the chunk
            let mut map: HashMap<String, StationData> = HashMap::new();
            for line in Cursor::new(buf).lines() {
                let line = line.unwrap();
                let mut parts = line.split(';');
                let station = parts.next().unwrap();
                let measurement = parts.next().unwrap().parse::<f32>().unwrap();

                if let Some(station_data) = map.get_mut(station) {
                    station_data.push(measurement);
                } else {
                    let station_data = StationData::new(measurement);
                    map.insert(station.to_owned(), station_data);
                }
            }


            map
        }).collect();

        // Merge all the maps together
//        while maps.len() > 1 {}

        // Alphabetically sort the station data
        let map = &maps[0];
        let mut stations: Vec<StationInfo> = map.into_iter().par_bridge().map(|(key, val)| StationInfo::new(key.to_string(), val.min, val.max, val.avg())).collect();
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

/// Find all the starting indices of chunks
fn find_chunk_boundaries(input: &Path) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let mut chunk_start_indices = [None; NUM_CHUNKS];
    chunk_start_indices[0] = Some(0);

    // Compute the approximate size of each chunk based on the size of the file
    let file_bytes = File::open(input)?.metadata()?.len();
    let bytes_per_step = (file_bytes as f32 / NUM_CHUNKS as f32).floor() as usize;

    // Adapt the chunk building code from src/chunks.rs to find the exact chunk boundaries
    let mut f = File::open(input)?;
    for i in 0..(NUM_CHUNKS - 1) {
        // Load the next chunk & find its boundary
        let mut buf = vec![0; bytes_per_step];
        let boundary = match f.read_exact(&mut buf) {
            // Case 1: there are enough bytes remaining in the file to fill the buffer
            Ok(_) => {
                // Find the last newline in the chunk in so we don't split a line across chunks
                let end_idx = last_chunk_newline_idx(&buf);

                // Move the cursor just past the newline in the chunk so the next chunk starts at
                // the beginning of a line rather than partway through one
                f.seek(SeekFrom::Current(end_idx as i64))?;

                // Return the current cursor position as the beginning of the next chunk
                Some(f.stream_position()? as usize)
            }
            // Case 2: there was an unexpected error
            Err(e) => match e.kind() {
                // Case 2: There are fewer bytes remaining in the file than required to fill the
                // buffer, but we still want to record the current chunk start index
                io::ErrorKind::UnexpectedEof => Ok(Some(f.stream_position()? as usize)),

                // Case 3: There was a real error
                _ => Err(e),
            }?,
        };


        // Save the computed chunk boundary
        chunk_start_indices[i + 1] = boundary;
    }

    // Add the current cursor position as the start index for the final chunk (since we don't
    // examine it during the loop)
    chunk_start_indices[NUM_CHUNKS - 1] = Some(f.stream_position()? as usize);

    println!("{:?}", chunk_start_indices);

    Ok(chunk_start_indices
        .iter().enumerate()
        .map(|(idx,opt)| opt.ok_or_else(|| format!("Missing chunk boundary for chunk {idx}!")))
        .collect::<Result<Vec<_>, _>>()?)
}
