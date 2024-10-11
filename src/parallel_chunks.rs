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
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

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

impl ChallengeRunner for Runner {
    fn run(input: &Path) -> ChallengeResult {
        let start = Instant::now();

        // Set up a thread pool using Rayon to process the chunks
        let pool = rayon::ThreadPoolBuilder::new().num_threads(PARALLEL_CHUNKS).build().unwrap();

        // Compute the approximate size of each chunk based on the size of the file
        let file_bytes = File::open(input)?.metadata()?.len();
        let bytes_per_step = (file_bytes as f32 / NUM_CHUNKS as f32).floor() as usize;

        // TODO: compute the exact chunk boundaries

        // TODO: spawn threads that will read the file & process the chunks in parallel

        todo!()
    }
}
