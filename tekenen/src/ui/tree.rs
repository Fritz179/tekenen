use std::{cell::RefCell, fmt::Display, rc::{Rc, Weak}};

#[derive(Debug)]
pub struct TreeData<T: Tree> {
    // parent
    parent: RefCell<Weak<T>>,

    // siblings
    next_sibling: RefCell<Option<Rc<T>>>,
    previous_sibling: RefCell<Weak<T>>,

    // children
    first_child: RefCell<Option<Rc<T>>>,
    last_child: RefCell<Weak<T>>,
}

pub struct TreeIterator<T: Tree>(Option<Rc<T>>);

impl<T: Tree> Iterator for TreeIterator<T> {
    type Item = Rc<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0.clone();
        if let Some(ref current) = current {
            self.0 = current.next_sibling();
        }
        current
    }
}

impl<T: Tree> IntoIterator for TreeData<T> {
    type Item = Rc<T>;
    type IntoIter = TreeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIterator(self.first_child.borrow().clone())
    }
}

impl<T: Tree> IntoIterator for &TreeData<T> {
    type Item = Rc<T>;
    type IntoIter = TreeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIterator(self.first_child.borrow().clone())
    }
}


impl<T: Tree + Display> Display for TreeData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = f.width().unwrap_or(0);

        // Write childrend, indenting by 4
        for child in self.iter() {
            writeln!(f)?;
            write!(f, "{child:width$}", width = width + 4)?;
        }

        Ok(())
    }
}

impl<T: Tree> Default for TreeData<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Tree> TreeData<T> {
    pub fn new() -> Self {
        Self {
            parent: RefCell::new(Weak::new()),
            next_sibling: RefCell::new(None),
            previous_sibling: RefCell::new(Weak::new()),
            first_child: RefCell::new(None),
            last_child: RefCell::new(Weak::new()),
        }
    }

    pub fn is_orphan(&self) -> bool {
        self.parent.borrow().upgrade().is_none()
            && self.next_sibling.borrow().is_none() 
            && self.previous_sibling.borrow().upgrade().is_none() 
    }

    pub fn iter(&self) -> &TreeData<T> {
        self
    }
}

pub trait Tree: Sized {
    fn get_data(&self) -> &TreeData<Self>;

    fn first_child(&self) -> Option<Rc<Self>> {
        self.get_data().first_child.borrow().clone()
    }

    fn last_child(&self) -> Option<Rc<Self>> {
        self.get_data().last_child.borrow().upgrade()
    }

    fn next_sibling(&self) -> Option<Rc<Self>> {
        self.get_data().next_sibling.borrow().clone()
    }

    fn append_child(self: &Rc<Self>, child: Rc<Self>) {
        let self_data = self.get_data();
        let child_data = child.get_data();

        // Child cannot already have a parent
        assert!(child_data.is_orphan());
        child_data.parent.replace(Rc::downgrade(self));

        // last sibling new next sibling
        if let Some(last_sibling) = self_data.last_child.borrow().upgrade() {
            last_sibling.get_data().next_sibling.replace(Some(child.clone()));
        }

        // child last sibling
        child_data.previous_sibling.replace(self_data.last_child.borrow().clone());

        // new last child
        self_data.last_child.replace(Rc::downgrade(&child));

        // maybe it was the first child
        if self_data.first_child.borrow().is_none() {
            self_data.first_child.replace(Some(child));
        }
    }

    fn has_children(&self) -> bool {
        self.get_data().first_child.borrow().is_some()
    }
}

// #[derive(Debug)]
// struct Node {
//     data: TreeData<Node>,
//     value: i32,
// }

// impl Tree for Node {
//     fn get_data(&self) -> &TreeData<Node> {
//         &self.data
//     }
// }

// impl Display for Node {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let width = f.width().unwrap_or(0);

//         write!(f, "{:width$}value: {}", "", self.value)?;
//         write!(f, "{:width$}", self.data)
//     }
// }

// impl Node {
//     fn new(value: i32) -> Rc<Self> {
//         Rc::new(Node {
//             data: TreeData::new(),
//             value
//         })
//     }
// }

// pub fn run() {
//     let node = Node::new(5);

//     node.append_child(Node::new(6));
//     node.append_child(Node::new(7));
//     node.append_child(Node::new(8));


//     println!("PRE");
//     println!("{}", node);
//     println!("POST");

//     println!("Hello, world!");
// }