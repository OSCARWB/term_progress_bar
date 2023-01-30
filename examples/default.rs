use term_progress_bar::{ProgressBar, bars::default_bar::DefaultBarBuilder};

fn main()
{
	let bar = DefaultBarBuilder::default()
	.name("Example")
	.bar_length(10)
	.build();
	let mut progbar: ProgressBar = ProgressBar::new(Box::new(bar));
	//println!("{:?}",progbar);
	loop
	{
		progbar.step_by(0.125);
		if progbar.is_done()
		{
			progbar.draw();
			break;
		}
		std::thread::sleep(std::time::Duration::from_secs_f64(0.1));
	}
	println!();
}