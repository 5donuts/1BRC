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

use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::fmt::Display;
use std::io;
use std::time::Duration;

/// A helper type to represent min/max/avg data for a station
#[derive(Debug)]
pub struct StationInfo((String, f32, f32, f32));

impl StationInfo {
    pub fn new(name: String, min: f32, max: f32, avg: f32) -> Self {
        Self((name, min, max, avg))
    }

    pub fn name(&self) -> &str {
        &self.0 .0
    }

    pub fn min(&self) -> f32 {
        self.0 .1
    }

    pub fn max(&self) -> f32 {
        self.0 .2
    }

    pub fn avg(&self) -> f32 {
        self.0 .3
    }
}

impl Display for StationInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}={:.1}/{:.1}/{:.1}",
            self.name(),
            self.min(),
            self.avg(),
            self.max()
        )
    }
}

// Only compare using the station name since all that matters is alphabetical ordering in the end
impl PartialEq for StationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for StationInfo {}

impl PartialOrd for StationInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name().partial_cmp(other.name())
    }
}

impl Ord for StationInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("Unable to unwrap StationInfo::partial_cmp")
    }
}

/// Helper type to represent the result of attempting the 1BRC Challenge.
///
/// When `Ok`, get the list of alphabetically-sorted [`StationInfo`] and a [`Duration`]
/// representing the amount of time it took to produce that result.
/// Otherwise, get the [`Error`](std::error::Error) encountered while computing the result.
pub type ChallengeResult = Result<(Vec<StationInfo>, Duration), Box<dyn std::error::Error>>;

pub trait ChallengeRunner {
    /// Solve the 1 Billion Row Challenge
    ///
    /// # Parameters
    /// * `input` - [`Path`] to the file containing the challenge input
    ///
    /// # Returns
    /// A [`Duration`] indicatating how long it took to solve the challenge,
    /// not including the amount of time it took to print the output, or some
    /// error encountered while attempting to solve the challenge.
    fn run<R>(input: R) -> ChallengeResult
    where
        R: io::Read + io::Seek;
}
