use term_progress_bar::{self, ProgressBarBuilder};

fn main()
{
	let mut progbar = ProgressBarBuilder::default()
		.name("Fred")
		.build()
		.unwrap();
	println!("{:?}",progbar);
	while !progbar.is_done()
	{
		progbar.step_by(0.1);
		std::thread::sleep(std::time::Duration::from_secs_f64(0.05));
	}
}