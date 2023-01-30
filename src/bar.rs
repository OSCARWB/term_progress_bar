use std::{fmt::Debug, io::Write};

use crate::Progress;

pub trait Bar : Debug
{
	fn draw_bar(&self,progress: &Progress)
	{
		let string = self.draw_bar_string(progress);
		print!("\r{}",string);
		let _res = std::io::stdout().flush();
	}

	fn draw_bar_string(&self,progress: &Progress) -> String;
}