use std::io::Write;

use crate::{bar::Bar, Progress};

#[derive(Debug)]
pub struct SimpleBar
{
	bar_length: usize
}

impl SimpleBar
{
	pub fn new() -> Self
	{
		Self {
			bar_length: 10,
		}
	}
}

impl Default for SimpleBar
{
	fn default() -> Self{
		Self::new()
	}
}

impl Bar for SimpleBar
{
	fn draw_bar(&self, progress: &Progress)
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
		// if string.len() != self.bar_length+2
		// {
		// 	eprintln!("prop: {}",progress.calc_proportion());
		// 	eprintln!("bars: {}",bars);
		// 	eprintln!("bar length: {}",self.bar_length);
		// 	eprintln!("bld: {}",self.bar_length-bars);
		// 	eprintln!("str len: {}", string.len());
		// }
		print!("\r{}",string);
		let _res = std::io::stdout().flush();
	}
}