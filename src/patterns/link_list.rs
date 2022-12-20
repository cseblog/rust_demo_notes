#[derive(Debug)]
struct Node {
    data: u32,
    next: Option<Box<Node>>
}

#[derive(Debug)]
struct LinkList {
    root: Option<Box<Node>>,
}

impl LinkList {
    fn insert(&mut self, value: u32) {
        let node = Node{data: value, next: self.root};
        self.root = Some(Box::new(node));    
    }
}

fn main() {
    println!("Hello link list");
    
}