use cached::instant::now;

mod tasks;

fn main()
{
    let start = now();
    println!("{}", tasks::p22::get_password("input/p22.txt"));
    println!("Time: {}s", (now() - start)/1000.0);
}