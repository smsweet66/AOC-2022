use cached::instant::now;

mod tasks;

fn main()
{
	let start = now();
	println!("{}", tasks::p25::sum_base_5("input/p25.txt"));
	println!("Time: {}s", (now() - start)/1000.0);
}