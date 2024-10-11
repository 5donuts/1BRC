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

        // Compute the approximate size of each chunk based on the size of the file
        let file_bytes = File::open(input)?.metadata()?.len();
        let bytes_per_step = (file_bytes as f32 / NUM_CHUNKS as f32).floor() as usize;

        // Set up the thread pool to process chunks
        let pool = ThreadPool::new(PARALLEL_CHUNKS);

        todo!()
    }
}

// --- The following is an implementation of a thread pool from The Book: https://doc.rust-lang.org/book/ch20-02-multithreaded.html --

/// A task for one of the [Worker]s in the [ThreadPool] to execute
type Job = Box<dyn FnOnce(String) -> HashMap<String, StationData> + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Initialize a new thread pool with `size` number of workers
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the number of workers is zero
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    /// Use a worker in the pool to execute the given closure
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(String) -> HashMap<String, StationData> + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });

        Self { thread }
    }
}
