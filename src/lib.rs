use std::{fmt::Debug};

pub mod progress;
use progress::Progress;

pub mod bar;
use bar::Bar;

pub mod bars;
use bars::{default_bar::DefaultBar};

pub fn add(left: usize, right: usize) -> usize 
{
	left + right
}

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

	pub fn new(bar: Box<dyn Bar>) -> Self
	{
		Self { bar: bar, progress: Default::default() }
	}
}

impl Default for ProgressBar
{
	fn default() -> Self
	{
		Self {
			progress: Default::default(),
			bar: Box::new(DefaultBar::new()),
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn it_works()
	{
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
