use cached::instant::now;

mod tasks;

fn main()
{
    let start = now();
    println!("{}", tasks::p20::get_sum_indices_keyed("input/p20.txt"));
    println!("Time: {}s", (now() - start)/1000.0);
}