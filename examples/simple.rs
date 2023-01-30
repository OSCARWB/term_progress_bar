use term_progress_bar::{ProgressBar, bars::simple_bar::SimpleBar};

fn main()
{
	let mut progbar: ProgressBar = ProgressBar::new(Box::new(SimpleBar::new()));
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