#[allow(dead_code)]
#[derive(Debug)]
pub struct Progress
{
	pub val_min: f64,
	pub val: f64,
	pub val_max: f64,
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