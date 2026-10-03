#[derive(Debug, PartialEq)]
pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, new_value: i32) {
        if self.value == new_value {
            return;
        } else if new_value < self.value {
            if self.left.is_none() {
                self.left = Some(Box::new(Node::new(new_value)));
                return;
            }
            self.left.as_mut().unwrap().insert(new_value);
        } else {
            if self.right.is_none() {
                self.right = Some(Box::new(Node::new(new_value)));
                return;
            }
            self.right.as_mut().unwrap().insert(new_value);
        }
    }
}

fn main() {
    let mut root = Node::new(10);
    root.insert(5);
    root.insert(15);
    root.insert(3);

    println!("Tree structure: {:#?}", root);

    // Assertions to test your solution
    assert_eq!(root.value, 10);
    assert_eq!(root.left.as_ref().unwrap().value, 5);
    assert_eq!(root.right.as_ref().unwrap().value, 15);
    assert_eq!(root.left.as_ref().unwrap().left.as_ref().unwrap().value, 3);
    assert!(root.left.as_ref().unwrap().right.is_none());

    println!("\nSuccess! All assertions passed.");
}