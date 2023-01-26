use std::{fmt::Debug, io::Write};
mod pb2;

pub fn add(left: usize, right: usize) -> usize {
	left + right
}

pub trait Bar : Debug
{
	fn draw_bar(&self,progress: &Progress);
}

#[derive(Debug)]
pub struct TestBar
{
	bar_length: i64
}

impl TestBar
{
	pub fn new() -> Self
	{
		Self {
			bar_length: 10,
		}
	}
}

impl Bar for TestBar
{
	fn draw_bar(&self, progress: &Progress)
	{
		let mut string: String = "[".into();
		let bars = (progress.calc_proportion()*self.bar_length as f64).round() as i64 + 1;
		eprintln!("prop: {}",progress.calc_proportion());
		eprintln!("bars: {}",bars);
		eprintln!("bar length: {}",self.bar_length);

		for _ in 0..bars
		{
			string += "=";
		}

		if progress.val <= progress.val_max
		{
			string += ">";
		}else{
			string += "=";
		}
		println!("{}",self.bar_length-bars);
		for _ in 0..(self.bar_length-bars)
		{
			string += " ";
		}
		string+="]";
		eprintln!("str len: {}", string.len());
		print!("\r{}\n",string);
		std::io::stdout().flush();
	}
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Progress
{
	val_min: f64,
	val: f64,
	val_max: f64,
}

impl Default for Progress
{
	fn default() -> Self
	{
		Self {
			val_min: 0.0,
			val: 0.0,
			val_max: 10.0,
		}
	}	
}

impl Progress
{
	pub fn calc_proportion(&self) -> f64
	{
		self.val/(self.val_max-self.val_min)
	}
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ProgressBar
{
	bar: Box<dyn Bar>,
	progress: Progress,
}

impl ProgressBar
{
	pub fn draw(&self)
	{
		self.bar.draw_bar(&self.progress);
	}

	pub fn step_by(&mut self, val: f64)
	{
		self.progress.val += val;
		if self.progress.val > self.progress.val_max
		{
			self.progress.val = self.progress.val_max;
		}
		self.draw();
	}

	pub fn step_to(&mut self, val: f64)
	{
		self.progress.val = val;
		self.draw();
	}

	pub fn is_done(&self) -> bool
	{
		self.progress.val >= self.progress.val_max
	}
}

impl Default for ProgressBar
{
	fn default() -> Self
	{
		Self {
			progress: Default::default(),
			bar: Box::new(TestBar::new()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
