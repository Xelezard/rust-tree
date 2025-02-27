use std::fmt::Debug;
use regex::Regex;
#[derive(Debug,Clone)]
pub struct Root<T> where T:Debug {
	pub name: String,
	pub value: Val<T>,
	pub roots: Vec<Root<T>>
}

#[derive(Debug,Clone,PartialEq)]
pub enum Val<T> where T: Debug {
	Rootpoint(String),
	Val(T)
}
pub fn create_tree<T,Q>(name: Q)-> Root<T>where T: Debug, String: From<Q> {
		Root{name: "root".to_string(),value: Val::Rootpoint(String::from(name)), roots: Vec::new()}
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
	pub fn get_value(&mut self) -> Option<&T> {
		if let Val::Val(v) = &self.value  {
			return Some(v);
		}
		None
	}
	pub fn get_child(&mut self,name: &str) -> Option<&mut Root<T>> {
		if name == "root" {
			return Some(self);
		}
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
impl Root<String> {
	pub fn from_tree_file(file_path: &str) -> Result<Root<String>,std::io::Error>{
		let file = std::fs::read_to_string(file_path)?;
		let mut result: Root<String> = create_tree("root");
		let re = Regex::new(r"((?:[ \t]*\|)*)[ \t]*(.+) -> (.+)").unwrap();
		let caps = re.captures_iter(&file);
		let mut last: Vec<String> = vec![String::from("root")];
		for cap in caps {
			let indents = &cap[1].chars().filter(|c|*c == '|').count();
			result.get_child(&last[*indents]).unwrap().append_child(Root::new(&cap[2], cap[3].to_string()));
			if last.len() <= *indents + 1 {
				last.push(String::from(&cap[2]));
			} else {
				last[*indents + 1] = String::from(&cap[2]);	
			}
		}
		Ok(result)
	}
}