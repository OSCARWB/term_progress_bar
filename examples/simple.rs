use term_progress_bar::{ProgressBar};

fn main()
{
	let mut progbar = ProgressBar::default();
	println!("{:?}",progbar);
	while !progbar.is_done()
	{
		progbar.step_by(1.0);
		std::thread::sleep(std::time::Duration::from_secs_f64(0.05));
	}
}