use term_progress_bar::{ProgressBar, TestBar};

fn main()
{
	let mut progbar: ProgressBar<TestBar> = ProgressBar::default();
	println!("{:?}",progbar);
	loop
	{
		progbar.step_by(0.25);
		if progbar.is_done()
		{
			progbar.draw();
			break;
		}
		std::thread::sleep(std::time::Duration::from_secs_f64(0.1));
	}
	println!();
}