use std::fmt::Debug;
#[derive(Debug,Clone)]
pub struct Root<T> where T:Debug {
	pub name: String,
	pub value: Val<T>,
	pub roots: Vec<Root<T>>
}

#[derive(Debug,Clone)]
pub enum Val<T> where T: Debug{
	Rootpoint,
	Val(T)
}

pub fn create_tree<T>() -> Root<T>where T: Debug {
		Root{name: "root".to_string(),value: Val::Rootpoint, roots: Vec::new()}
}
impl<T> Root<T> where T: Debug{
	pub fn new(name: &str,value: T) -> Root<T>{
		Root {name: name.to_string(),value: Val::Val(value),roots: Vec::new()}
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
	pub fn append_child(&mut self,child: Root<T>) -> &mut Self {
		self.roots.push(child);
		self
	}
	pub fn change_name(&mut self,name:&str) -> &mut Self {
		self.name = name.to_string();
		self
	}
	pub fn change_value(&mut self,value:T) -> &mut Self {
		self.value = Val::Val(value);
		self
	}
	pub fn get_child(&mut self,name: &str) -> Option<&mut Root<T>> {
		for i in 0..self.roots.len() {
			if self.roots[i].name == name {
				return Some(&mut self.roots[i])
			}
			for j in 0..self.roots[i].roots.len() {
				if self.roots[i].roots[j].name == name {
					return Some(&mut self.roots[i].roots[j])
				}
			}
		}
		None
	}
}