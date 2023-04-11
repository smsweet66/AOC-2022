use cached::instant::now;

mod tasks;

fn main()
{
    let start = now();
    println!("{}", tasks::p24::minimum_moves_round_trip("input/p24.txt"));
    println!("Time: {}s", (now() - start)/1000.0);
}