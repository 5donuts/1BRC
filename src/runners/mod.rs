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

mod baseline;
mod chunks;

pub use baseline::Runner as Baseline;
pub use chunks::Runner as Chunks;

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::prelude::*;

    use once_cell::sync::Lazy;

    use crate::helpers::*;
    use super::*;

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

    #[test]
    fn correctness() -> Result<(), Box<dyn std::error::Error>> {
        // Write the test data to a file
        let fname = std::path::Path::new("test.txt");
        let mut file = File::create(fname)?;
        write!(file, "{TEST_DATA}")?;

        // Test each runner for correctness with this small set of test data
        let (actual, _) = Baseline::run(fname)?;
        assert_eq!(actual, *EXPECTED_RESULT, "Error in baseline runner");

        let (actual, _) = Chunks::run(fname)?;
        assert_eq!(actual, *EXPECTED_RESULT, "Error in chunks runner");

        // Delete the test file
        fs::remove_file(fname)?;

        Ok(())
    }
}
