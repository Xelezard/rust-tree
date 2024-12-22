use std::sync::Arc;
use std::fmt::Debug;
#[derive(Debug,Clone)]
pub struct Root{
	pub name: String,
	pub value: Val,
	pub roots: Vec<Root>
}
#[derive(Debug,Clone)]
pub enum Val {
	Root(Rootpoint),
	Val(Arc<dyn Debug>),
}
#[derive(Debug,Clone)]
pub struct Rootpoint;
pub fn create_tree() -> Root {
		Root{name: "root".to_string(),value: Val::Root(Rootpoint), roots: Vec::new()}
}
impl Root{
	pub fn new(name: &str,value: impl Debug + 'static) -> Root{
		Root {name: name.to_string(),value: Val::Val(Arc::new(value)),roots: Vec::new()}
	}
	pub fn clone(&self) -> Root{
		Root {name: self.name.clone(),value:self.value.clone(),roots:self.roots.clone()}
	}
	pub fn show(&self,tabs: u8) -> &Self {
		let mut tabstring = String::new();
		for _ in 0..tabs {
			tabstring += "\t";
		}
		println!("{}{}: {:?}",tabstring,self.name,self.value);
		for root in &self.roots {
			root.show(tabs + 1);
		}
		self
	}
	pub fn append_child(&mut self,child: Root) -> &mut Self {
		self.roots.push(child);
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