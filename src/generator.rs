extern crate fakeit;

use crate::models::Object;
use fakeit::words;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::time::{Duration, Instant};

impl Object {
    /// Generate a randomaized mock object
    pub fn mock() -> (Object, Duration) {
        // Generate a mock for object which each field will be randomized based on type.
        let start = Instant::now();
        let mut rng = rand::thread_rng();

        let id = rng.gen_range(0..1_000_000_000);
        let name = words::word();
        let index = rng.gen_range(0..1_000_000);
        let code = (0..15).map(|_| rng.sample(Alphanumeric) as char).collect();

        (
            Object {
                id,
                name,
                index,
                code,
            },
            start.elapsed(),
        )
    }

    /// Generate list of randomaized mock objects
    pub fn mocks(count: i32) -> (Vec<Object>, Duration) {
        let start = Instant::now();
        let mut mocks: Vec<Object> = Vec::new();

        for _i in 0..count {
            let mut rng = rand::thread_rng();

            let id = rng.gen_range(0..1_000_000_000);
            let name = words::word();
            let index = rng.gen_range(0..1_000_000);
            let code = (0..15).map(|_| rng.sample(Alphanumeric) as char).collect();

            mocks.push(Object {
                id,
                name,
                index,
                code,
            });
        }

        (mocks, start.elapsed())
    }
}
