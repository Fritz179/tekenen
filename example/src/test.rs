use std::{fmt::Display, rc::Rc};

use tekenen::ui::tree::{Tree, TreeData};

#[derive(Debug)]
struct Node {
    data: TreeData<Node>,
    value: i32,
}

impl Tree for Node {
    fn get_data(&self) -> &TreeData<Node> {
        &self.data
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = f.width().unwrap_or(0);

        write!(f, "{:width$}value: {}", "", self.value)?;
        write!(f, "{:width$}", self.data)
    }
}

impl Node {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Node {
            data: TreeData::new(),
            value
        })
    }
}

pub fn run() {
    let node = Node::new(5);

    node.append_child(Node::new(6));
    node.append_child(Node::new(7));
    node.append_child(Node::new(8));

    for child in node.data.iter() {
        println!("A{}B", child);
    }

    println!("PRE");
    println!("{}", node);
    println!("POST");

    println!("Hello, world!");
}