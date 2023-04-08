use cached::instant::now;

mod tasks;

fn main()
{
    let start = now();
    println!("{}", tasks::p23::get_first_empty_round("input/p23.txt"));
    println!("Time: {}s", (now() - start)/1000.0);
}