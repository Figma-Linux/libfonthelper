use std::collections::HashMap;

pub type FontMap = HashMap<String, FontEntry>;

pub struct FontEntry {
    pub postscript: String,
	pub family: String,
	pub id: String,
	pub style: String,
	pub weight: i64,
	pub stretch: i64,
	pub italic: bool,
}
