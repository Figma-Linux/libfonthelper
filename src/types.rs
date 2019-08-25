use std::collections::HashMap;

pub type FontMap = HashMap<String, Vec<FontEntry>>;

pub struct FontEntry {
    pub postscript: String,
	pub family: String,
	pub id: String,
	pub style: String,
	pub weight: i32,
	pub stretch: i32,
	pub italic: bool,
}
