// pub trait Printer
// {

// }

// #[allow(dead_code)]
// #[derive(Builder,Debug)]
// #[builder(default)]
// pub struct ProgressBar
// {
// 	#[builder(default = "0f64")]
// 	val_min:f64,
// 	#[builder(default = "0f64")]
// 	pub val: f64,
// 	#[builder(default = "10f64")]
// 	val_max: f64,


// 	#[builder(default = "10")]
// 	bar_length: usize,

// 	#[builder(setter(into, strip_option))]
// 	name: Option<String>,
// }

// #[allow(dead_code)]
// impl Default for ProgressBar
// {
// 	fn default() -> Self {
// 		Self { 
// 			val_min: 0f64,
// 			val: 0f64,
// 			val_max: 10f64,
// 			bar_length: 10,
// 			name: Default::default(),
// 		}
// 	}
// }

// #[allow(dead_code)]
// impl ProgressBar
// {
// 	fn calc_bar(&self,percent: f64) -> String
// 	{
// 		let mut string: String = "".into();
// 		let bars = (percent*self.bar_length as f64/100.0).round() as usize;
// 		for _ in 0..bars
// 		{
// 			string += "=";
// 		}
// 		if self.val < self.val_max
// 		{
// 			string += ">";
// 		}else{
// 			string += "=";
// 		}
// 		for _ in 0..(self.bar_length-bars)
// 		{
// 			string += " ";
// 		}
// 		return string;
// 	}

// 	fn calc_percent(percent_int:i8) -> String
// 	{
// 		let mut string: String = " ".into();
// 		if percent_int / 10 == 0
// 		{
// 			string += " ";
// 		}
// 		if percent_int / 100 == 0
// 		{
// 			string += " ";
// 		}
// 		string += &percent_int.to_string();
// 		string += "% ";
// 		return string;
// 	}

// 	pub fn print(&self)
// 	{
// 		let percent = self.val/self.val_max*100.0;
// 		let int_percent = percent as i8;
// 		let mut string = String::new();
// 		string += "\r";
// 		if let Some(name) = &self.name { string += name };
// 		string += ":";
// 		string += &Self::calc_percent(int_percent);
// 		string += "[";
// 		string += &self.calc_bar(percent);
// 		string += "]";
// 		print!("{}",string);
// 		std::io::Write::flush(&mut std::io::stdout()).expect("Could not flush io");
// 	}

// 	pub fn step_by(&mut self, val: f64)
// 	{
// 		self.val += val;
// 		self.print();
// 	}

// 	pub fn step_to(&mut self, val: f64)
// 	{
// 		self.val = val;
// 		self.print();
// 	}

// 	pub fn is_done(&self) -> bool
// 	{
// 		self.val >= self.val_max
// 	}
// }