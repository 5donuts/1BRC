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
use std::io::{BufRead, BufReader};
use std::time::Instant;

use crate::helpers::*;

pub struct Runner;

/// The size of the buffer (in bytes) to use while reading the input
const BUFFER_SIZE: usize = 8_589_934_592; // 8 GiB

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

        // Configure our BufReader to use a larger buffer so we make fewer I/O operations while
        // reading the file. Otherwise, this approach is identical to the one in src/baseline.rs
        let mut map: HashMap<String, StationData> = HashMap::new();
        for line in BufReader::with_capacity(BUFFER_SIZE, input).lines() {
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

/// Function to read the contents of a buffer containing data read from the input source and
/// split that data into two slices:
/// 1. a slice containing only complete lines from the input source
/// 2. a slice containing a partial line from the input source
///
/// This way, we can use [`BufRead::consume`] to mark all data in the buffer as read and load
/// new data from the input source while still preserving the partial input read from the
/// initial call to [`BufRead::fill_buf`].
///
/// The idea here is that it's possible that we stop reading data before we've read a complete
/// line from the input source and we don't want to attempt to process that data until we have
/// the rest of that line.
///
/// # Assumptions
/// This function assumes that `input` is non-empty and will panic otherwise.
///
/// # Returns
/// A tuple containing two slices:
/// 1. a slice containing only complete lines from the input source
/// 2. a slice containing a partial line from the input source
///
/// If there are no partial lines from the input source, the second slice will be empty and the
/// first will contain the entirety of the `input`.
///
/// If there are no newlines in the input source, the first slice will be empty and the second
/// will contain the entirety of the `input`.
fn get_measurement_lines(input: &[u8]) -> (&[u8], &[u8]) {
    // Get the index of the last newline byte in the input.
    // All bytes up to and including this point are 'full' lines.
    // All bytes after this point are 'partial' lines.
    // If there are no '\n' bytes in the input, then the entire input must be a partial line
    if let Some(last_newline_idx) = input
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(idx, c)| if *c == b'\n' { Some(idx) } else { None })
        .next()
    {
        let full_lines = &input[..=last_newline_idx];
        let partial_lines = if last_newline_idx == input.len() {
            &[]
        } else {
            &input[last_newline_idx + 1..]
        };

        (full_lines, partial_lines)
    } else {
        let full_lines = &[];
        let partial_lines = &input;

        (full_lines, partial_lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runners::tests::*;
    use std::{error, io};

    #[test]
    fn correctness() -> Result<(), Box<dyn error::Error>> {
        let input = io::Cursor::new(TEST_DATA.as_bytes());

        let (actual, _) = Runner::run(input)?;
        assert_eq!(
            actual, *EXPECTED_RESULT,
            "actual != expected for chunks runner"
        );

        Ok(())
    }

    /// Tests for the [`get_measurement_lines`] function
    mod get_measurement_lines {
        use super::*;

        #[test]
        fn only_full_lines() {
            let input = r#"Eslohe;6.6
Anta;-98.3
Okegawa;-45.1
General Pinedo;-67.7
Cavarzere;5.0
Pongoz;38.9
Douglasville;43.5
Vinjam;-51.1
Singalāndāpuram;72.1
Erandio;74.8
"#;
            let input = input.as_bytes();

            let expected_full_lines = r#"Eslohe;6.6
Anta;-98.3
Okegawa;-45.1
General Pinedo;-67.7
Cavarzere;5.0
Pongoz;38.9
Douglasville;43.5
Vinjam;-51.1
Singalāndāpuram;72.1
Erandio;74.8
"#;
            let expected_full_lines = expected_full_lines.as_bytes();
            let expected_partial_lines: &[u8] = &[];

            let (actual_full_lines, actual_partial_lines) = get_measurement_lines(input);
            assert_eq!(
                expected_full_lines, actual_full_lines,
                "Full lines: expected != actual"
            );
            assert_eq!(
                expected_partial_lines, actual_partial_lines,
                "Partial lines: expected != actual"
            );
        }

        #[test]
        fn only_partial_line() {
            let input = r#"Eslohe;"#;
            let input = input.as_bytes();

            let expected_full_lines: &[u8] = &[];
            let expected_partial_lines = b"Eslohe;";

            let (actual_full_lines, actual_partial_lines) = get_measurement_lines(input);
            assert_eq!(
                expected_full_lines, actual_full_lines,
                "Full lines: expected != actual"
            );
            assert_eq!(
                expected_partial_lines, actual_partial_lines,
                "Partial lines: expected != actual"
            );
        }

        #[test]
        fn full_and_partial_lines() {
            let input = r#"Eslohe;6.6
Anta;-98.3
Okegawa;-45.1
General Pinedo;-67.7
Cavarzere;5.0
Pongoz;38.9
Douglasville;43.5
Vinjam;-51.1
Singalāndāpuram;"#;
            let input = input.as_bytes();

            let expected_full_lines = r#"Eslohe;6.6
Anta;-98.3
Okegawa;-45.1
General Pinedo;-67.7
Cavarzere;5.0
Pongoz;38.9
Douglasville;43.5
Vinjam;-51.1
"#;
            let expected_full_lines = expected_full_lines.as_bytes();

            let expected_partial_lines = r#"Singalāndāpuram;"#;
            let expected_partial_lines = expected_partial_lines.as_bytes();

            let (actual_full_lines, actual_partial_lines) = get_measurement_lines(input);
            assert_eq!(
                expected_full_lines, actual_full_lines,
                "Full lines: expected != actual"
            );
            assert_eq!(
                expected_partial_lines, actual_partial_lines,
                "Partial lines: expected != actual"
            );
        }
    }
}
