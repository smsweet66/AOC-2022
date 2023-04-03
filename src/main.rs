use cached::instant::now;

mod tasks;

fn main()
{
    let start = now();
    println!("{}", tasks::p19::get_sum_geodes("input/p19.txt"));
    println!("Time: {}s", (now() - start)/1000.0);
}