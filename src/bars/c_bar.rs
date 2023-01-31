use crate::{bar::Bar, Progress};

#[derive(Debug)]
pub struct CBar
{
	bar_length: usize,
	start_char: String,
	mid_char: String,
	head_char: String,
	space_char: String,
	end_char: String,
}

impl CBar
{
	pub fn new() -> Self
	{
		Self {
			bar_length: 10,
			start_char:	"[".into(),
			mid_char:	"=".into(),
			head_char:	">".into(),
			space_char:	" ".into(),
			end_char:	"]".into(),
		}
	}

	pub fn with_length(length: usize) -> Self
	{
		let mut new = Self::new();
		new.bar_length = length;
		new
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
		let mut string = self.start_char.clone();
		let bars = (progress.calc_proportion()*self.bar_length as f64).round() as usize;
		let mut count = 1;
		for _ in 1..bars
		{
			count+=1;
			string += &self.mid_char;
		}

		if progress.val < progress.val_max
		{
			string += &self.head_char;
		}else{
			string += &self.mid_char;
		}

		for _ in count..self.bar_length
		{
			string += &self.space_char;
		}
		string+= &self.end_char;

		string
	}
}