# Rust-Tree
Simple Tree data type for the rust programming language

# Usage

To create a new tree use 
```
let tree = create_tree()
```
Every Tree consist of multiple Roots, one being the "Root" Root
Every Root consist of its name, its value and its roots
```
pub struct Root{
	pub name: String,
	pub value: Val,
	pub roots: Vec<Root>
}
```

To visualize a tree use
```
tree.show(0) // the 0 is important
```
A brand new tree looks like this
```
root: Root
```
"root" is the name and "Root" is the type

To create a new Root use
```
let root: Root = Root::new("Name","Value") // All types that implement the Debug trait can be used as a value
```

To append the new root to the tree use:
```
tree.append_child(root)
```
The tree should now look like this
```
root: Root
        Name: Val("Value")
```

To acces a child root use
```
tree.get_child("child name")
```

To get the name of a root use
```
root.name
```
To change it use
```
root.change_name("new name")
```

To get the value of a root use
```
root.value
```
To change it use
```
root.change_value(new_value)
```

The "show()", "append_child()", "change_name()" and "change_value()" functions return the tree so that this is possible
```
tree
	.append_child(child1)
	.append_child(child2)
	.show(0);
```