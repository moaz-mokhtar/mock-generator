use std::{thread::sleep, time::Duration};

use mock_generator::models::Object;

fn main() {
    println!(" ===================================");
    println!(" ===| Welcome to Mock Generator |===");
    println!(" ===================================\n");

    println!("Single mock function");

    let (object, duration) = Object::mock();
    println!("|\tCost (nanoseconds): {:?}", duration.as_nanos());
    println!("|\tCost (seconds): {:?}", duration.as_secs_f64());
    println!("|\tMocked object: {:?}", object);

    println!(" === \n");
    sleep(Duration::from_secs(2));
    println!("Waiting 2 seconds");
    let count = 10;
    println!("List of mocks function. Count: {}", count);

    let (objects, duration) = Object::mocks(count);
    println!("|\tCost (nanoseconds): {:?}", duration.as_nanos());
    println!("|\tCost (seconds): {:?}", duration.as_secs_f64());
    println!("|\tMocked object: {:?}", objects);
}

/*
TODO Plan

*/


