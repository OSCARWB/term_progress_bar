use std::fmt::Debug;

use crate::Progress;

pub trait Bar : Debug
{
	fn draw_bar(&self,progress: &Progress);
}