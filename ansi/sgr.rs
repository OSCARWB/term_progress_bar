use std::ops::Add;

pub enum SGR
{
	Style(Style),
	Colour(Colour),
}

impl SGR
{
	#[allow(non_snake_case)]
	pub fn RGB(r: u8, g: u8, b:u8) -> Self
	{
		Self::Colour(Colour::new(r, g, b))
	}

	pub fn reset() -> Self
	{
		Self::Style(Style::Reset)
	}

	pub fn bold() -> Self
	{
		Self::Style(Style::Bold)
	}

	pub fn to_esc_string(self) -> String
	{

		match self {
			SGR::Style(s) => s.to_esc_string(),
			SGR::Colour(c) => c.to_esc_string(),
		}
	}
}

impl Add<&str> for SGR
{
	type Output = String;

	fn add(self, rhs: &str) -> Self::Output
	{
		self.to_esc_string() + rhs
	}
}

impl Add<SGR> for &str
{
	type Output = String;

	fn add(self, rhs: SGR) -> Self::Output
	{
		self.to_string() + rhs.to_esc_string().as_str()
	}
}

impl Add<SGR> for SGR
{
	type Output = String;

	fn add(self, rhs: SGR) -> Self::Output {
		self.to_esc_string() + rhs.to_esc_string().as_str()
	}
}

impl Add<Style> for SGR
{
	type Output = String;

	fn add(self, rhs: Style) -> Self::Output {
		self.to_esc_string() + rhs.to_esc_string().as_str()
	}
}

impl Add<SGR> for Style
{
	type Output = String;

	fn add(self, rhs: SGR) -> Self::Output {
		self.to_esc_string() + rhs.to_esc_string().as_str()
	}
}

pub enum Style
{
	Reset = 0,
	Bold = 1,
	Faint = 2,
	Italic = 3,
	Underline = 4,
	SlowBlink = 5,
	RapidBlink = 6,
	Invert = 7,
	Hide = 8,
	Strike = 9,
	PrimaryFont = 10,
	AltFont1 = 11,
	AltFont2 = 12,
	AltFont3 = 13,
	AltFont4 = 14,
	AltFont5 = 15,
	AltFont6 = 16,
	AltFont7 = 17,
	AltFont8 = 18,
	AltFont9 = 19,
	Fraktur = 20,
	DoubleUnderline = 21,
	NormalIntensity = 22,
	N23,
	N24,
	N25,
	N26,
	NotInvert,
	N28,
	NotStrike,
	N50 = 50,
	Framed,
	Encircled,
	Overlined,
	N54,
	NotOverlined,
}

impl Style
{
	pub fn to_esc_string(self) -> String
	{
		let mut string: String = "\x1b[".into();

		let num = self as u64;
		if num != 0
		{
			string += &num.to_string();
		}

		string += "m";
		string
	}
}

impl Add<Style> for &str
{
	type Output = String;

	fn add(self, rhs: Style) -> Self::Output
	{
		self.to_string() + rhs.to_esc_string().as_str()
	}
}

impl Add<&str> for Style
{
	type Output = String;

	fn add(self, rhs: &str) -> Self::Output
	{
		self.to_esc_string() + rhs
	}
}

impl Add<String> for Style
{
	type Output = String;

	fn add(self, rhs: String) -> Self::Output
	{
		self + rhs.as_str()
	}
}

impl Add<Style> for String
{
	type Output = Self;

	fn add(self, rhs: Style) -> Self::Output
	{
		self + rhs.to_esc_string().as_str()
	}
}

pub struct Colour
{
	r: u8,
	g: u8,
	b: u8,
}

impl Colour
{
	pub fn new (r: u8, g: u8, b:u8) -> Self
	{
		Self{
			r:r,
			g:g,
			b:b,
		}
	}

	pub fn to_esc_string(self) -> String
	{
		"\x1b[38;2;".to_string() + self.r.to_string().as_str() + ";" + self.g.to_string().as_str() + ";" + self.b.to_string().as_str() + "m"
	}
}

// impl Style
// {
// 	pub fn to_esc_code(self) -> String
// 	{
// 		let mut string: String = "\x1b[".into();

// 		let num = self as u64;
		
// 		string += &num.to_string();

// 		string += "m";
// 		string
// 	}
// }

// impl Add<Style> for Style
// {
// 	type Output = String;

// 	fn add(self, rhs: Self) -> Self::Output
// 	{
// 		self.to_esc_code() + rhs.to_esc_code().as_str()
// 	}
// }

// impl Add<String> for Style
// {
// 	type Output = String;

// 	fn add(self, rhs: String) -> Self::Output
// 	{
// 		self + rhs.as_str()
// 	}
// }

// impl Add<&str> for Style
// {
// 	type Output = String;

// 	fn add(self, rhs: &str) -> Self::Output
// 	{
// 		self.to_esc_code() + rhs
// 	}
// }

// impl Add<Style> for String
// {
// 	type Output = Self;

// 	fn add(self, rhs: Style) -> Self::Output
// 	{
// 		self + rhs.to_esc_code().as_str()
// 	}
// }

// impl Add<Style> for &str
// {
// 	type Output = String;

// 	fn add(self, rhs: Style) -> Self::Output
// 	{
// 		self.to_string() + rhs.to_esc_code().as_str()
// 	}
// }