use crate::{bar::Bar, Progress};

use super::simple_bar::SimpleBar;

#[derive(Debug)]
pub struct DefaultBar
{
	bar: SimpleBar,
	name: Option<String>
}

impl DefaultBar
{
	pub fn new() -> Self
	{
		Self
		{
			bar: Default::default(),
			name: Default::default(),
		}
	}
}

impl Bar for DefaultBar
{
	fn draw_bar_string(&self,progress: &Progress) -> String
	{
		let mut string = "".to_string();
		if let Some(name) = &self.name {
			string += name;
			string += " ";
		}


		{
			let mut ts = "".to_string();
			ts += &((progress.calc_proportion()*100.0) as i64).to_string();
			for _ in ts.len()..3
			{
				string += " ";
			}
			string += &ts;
		}
		string += "% ";

		string += &self.bar.draw_bar_string(progress);

		return string;
	}
}

impl Default for DefaultBar
{
	fn default() -> Self
	{
		Self::new()
	}
}

pub struct DefaultBarBuilder
{
	bar_length: usize,
	name: Option<String>,
}

impl DefaultBarBuilder
{
	pub fn new() -> Self
	{
		Self {
			bar_length: 10,
			name: None
		}
	}

	pub fn build(self) -> DefaultBar
	{
		DefaultBar {
			bar: SimpleBar::with_length(self.bar_length),
			name: self.name.clone(),
		}
	}

	pub fn name(mut self, name: &str) -> DefaultBarBuilder
	{
		self.name = Some(name.to_string());
		self
	}

	pub fn bar_length(mut self, length: usize) -> DefaultBarBuilder
	{
		self.bar_length = length;
		self
	}
}

impl Default for DefaultBarBuilder
{
	fn default() -> Self
	{
		Self::new()
	}
}