use std::sync::Arc;
use std::fmt::Debug;
#[derive(Debug,Clone)]
pub struct Root{
	pub name: String,
	pub value: Arc<dyn Debug>,
	pub roots: Vec<Root>
}

#[derive(Debug,Clone)]
struct Rootpoint;

pub fn create_tree() -> Root {
		Root{name: "root".to_string(),value: Arc::new(Rootpoint), roots: Vec::new()}
}
impl Root{
	pub fn new(name: &str,value: impl Debug + 'static) -> Root{
		Root {name: name.to_string(),value: Arc::new(value),roots: Vec::new()}
	}
	pub fn show(&mut self,tabs: u8) -> &mut Self {
		let mut tabstring = String::new();
		for _ in 0..tabs {
			tabstring += "\t";
		}
		println!("{}{}: {:?}",tabstring,self.name,self.value);
		for root in &mut self.roots {
			root.show(tabs + 1);
		}
		self
	}
	pub fn append_child(&mut self,child: Root) -> &mut Self {
		self.roots.push(child);
		self
	}
	pub fn change_name(&mut self,name:&str) -> &mut Self {
		self.name = name.to_string();
		self
	}
	pub fn change_value(&mut self,value: impl Debug + 'static) -> &mut Self {
		self.value = Arc::new(value);
		self
	}
	pub fn get_child(&mut self,name: &str) -> &mut Root {
		for i in 0..self.roots.len() {
			if self.roots[i].name == name {
				return &mut self.roots[i]
			}
		}
		panic!("No Child Found")
	}
}