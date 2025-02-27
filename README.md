# Rust-Tree
Simple Tree data type for the rust programming language

# Install
To install run
```
cargo add --git="https://github.com/Xelezard/rust-tree" tree
```
# Usage

To create a new tree use 
```rust
let tree:Root<type> = create_tree("Name")
```
Every Tree consist of multiple Roots, one being the "Root" Root
Every Root consist of its name, its value and its roots
```rust
pub struct Root<T>{
	pub name: String,
	pub value: Val<T>,
	pub roots: Vec<Root<T>>
}
```

To visualize a tree use
```rust
tree.show(0) // the 0 is important
```
A brand new tree looks like this
```
root: Root
```
"root" is the name and "Root" is the type

To create a new Root use
```rust
let root: Root = Root::new("Name","Value")
```

To append the new root to the tree use:
```rust
tree.append_child(root)
```
The tree should now look like this
```
root: Root
        Name: Val("Value")
```

To acces a child root use
```rust
tree.get_child("child name")
```

To get the name of a root use
```
root.name
```
To change it use
```rust
root.change_name("new name")
```

To get the value of a root use
```rust
root.get_value().unwrap()
```
To change it use
```rust
root.change_value(new_value)
```

The "show()", "append_child()", "change_name()" and "change_value()" functions return the tree so that this is possible
```rust
tree
	.append_child(child1)
	.append_child(child2)
	.show(0);
```
# Tree files
Tree files are file used to construct a 'Root<String>' tree

Create one like this:
```rust
let tree = Root::from_tree_file("path")?;
```
They are built up like this

```
key -> value
key2 -> value2
| subkey -> subvalue
|| subsubkey -> subsubvalue
| subkey2 -> subvalue2
```