use std::time::Instant;
use cached::instant::now;

mod tasks;

fn main()
{
    let start = now();
    println!("{}", tasks::p16::get_max_pressure("input/p16.txt"));
    println!("Time: {}ms", now() - start);
}