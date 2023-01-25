#[macro_use]
extern crate derive_builder;
mod pb2;

pub fn add(left: usize, right: usize) -> usize {
	left + right
}

pub struct Bar
{
	val_min: f64,
	val: f64,
	val_max: f64,
}


pub trait Progressor
{
	fn draw(&self);

	fn step_by(&mut self, val: f64);

	fn step_to(&mut self, val: f64);

	fn is_done(&self) -> bool;
}

#[allow(dead_code)]
#[derive(Builder,Debug)]
#[builder(default)]
pub struct ProgressBar
{
}

#[allow(dead_code)]
impl Progressor for ProgressBar
{
	fn draw(&self)
	{
		todo!()
	}

	fn step_by(&mut self, val: f64)
	{
		self.val += val;
		self.draw();
	}

	fn step_to(&mut self, val: f64)
	{
		self.val = val;
		self.draw();
	}

	fn is_done(&self) -> bool
	{
		self.val >= self.val_max
	}
}

#[allow(dead_code)]
impl Default for ProgressBar
{
	fn default() -> Self {
		Self {

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
