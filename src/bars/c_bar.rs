use crate::{bar::Bar, Progress};

#[derive(Debug)]
pub struct CBar
{
	bar_length: usize
}

impl CBar
{
	pub fn new() -> Self
	{
		Self {
			bar_length: 10,
		}
	}

	pub fn with_length(length: usize) -> Self
	{
		Self {
			bar_length: length,
		}
	}
}

impl Default for CBar
{
	fn default() -> Self{
		Self::new()
	}
}

impl Bar for CBar
{
	fn draw_bar_string(&self, progress: &Progress) -> String
	{
		let mut string: String = "[".into();
		let bars = (progress.calc_proportion()*self.bar_length as f64).round() as usize;

		for _ in 1..bars
		{
			string += "=";
		}

		if progress.val < progress.val_max
		{
			string += ">";
		}else{
			string += "=";
		}


		for _ in string.len()..self.bar_length+1
		{
			string += " ";
		}
		string+="]";

		string
	}
}