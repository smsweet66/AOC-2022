use cached::instant::now;

mod tasks;

fn main()
{
    let start = now();
    println!("{}", tasks::p14::get_num_sand_pieces_floored("input/p14.txt"));
    println!("Time: {}ms", now() - start);
}