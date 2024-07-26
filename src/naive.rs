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

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Instant;
use std::collections::HashMap;

use crate::helpers::*;

struct StationData {
    min: f32,
    max: f32,

    // Rather than compute a new average at each step, just keep a rolling sum
    // of all the measurements and calculate the average at the end.
    sum: f32,
    cnt: u32,
}

impl StationData {
    fn avg(&self) -> f32 {
        self.sum / self.cnt as f32
    }
}

pub struct Runner;

impl ChallengeRunner for Runner {
    fn run(input: &Path) -> ChallengeResult {
        let start = Instant::now();

        // A quick 'wc -L measurements.txt' suggests the longest line will be in the
        // realm of 45 characters. So, make the buffer long enough to hold a good number
        // of lines to reduce the number of I/O operations.
        const BUFFER_SIZE: usize = std::mem::size_of::<char>() * 50000;
        let mut buffer = [0; BUFFER_SIZE];
        let mut map: HashMap<String, StationData> = HashMap::new();

        let mut f = File::open(input)?;

        let mut n = f.read(&mut buffer[..])?;
        while n > 0 {
            // TODO: actually do something with the data lol
            n = f.read(&mut buffer[..])?;
        }

        // Build the alphabetically-sorted list of stations
        let stations = Vec::new(); // TODO

        // Compute the time it took to generate the list of sorted stations
        let stop = Instant::now();
        let duration = stop.duration_since(start);

        Ok((stations, duration))
    }
}
