use cached::instant::now;

mod tasks;

fn main()
{
    let start = now();
    println!("{}", tasks::p21::get_input_value("input/p21.txt"));
    println!("Time: {}s", (now() - start)/1000.0);
}