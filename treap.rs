#[derive(Debug)]
struct Treap<T> {
    node: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

type Priority = u32;

// Invariant: All values in 'left' are less than 'value' and all values in
// 'right' are greater than 'value'.
// The priority of each node is greater or equal to all its sub-nodes.
#[allow(dead_code)]
#[derive(Debug)]
struct Node<T> {
    priority: Priority,
    value: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord> Treap<T> {
    fn new() -> Self {
        Treap{ node: None }
    }

    fn insert(self: &mut Self, priority: Priority, value: T) {
        let leaf = Box::new(Node{ priority: priority, value: value, left: None, right: None });
        insert(&mut self.node, leaf);
    }
}

fn insert<T: Ord>(node: &mut Option<Box<Node<T>>>, leaf: Box<Node<T>>) {
    *node = match std::mem::replace(node, None) {
        Some(mut link) => {
            if leaf.value < link.value {
                insert(&mut link.left, leaf);
                fix_left(&mut link);
            } else {
                insert(&mut link.right, leaf);
                fix_right(&mut link);
            }
            Some(link)
        }
        None => Some(leaf),
    }
}

fn fix_left<T: Ord>(node: &mut Box<Node<T>>) {
    let priority = node.priority;
    if let Some(left_box) = std::mem::replace(&mut node.left, None) {
        assert!(left_box.value < node.value);
        if left_box.priority > priority {
            let mut right_box = std::mem::replace(node, left_box);
            right_box.left = std::mem::replace(&mut node.right, None);
            assert!(node.value < right_box.value);
            node.right = Some(right_box);
        } else {
            node.left = Some(left_box);
        }
    }
}

fn fix_right<T: Ord>(node: &mut Box<Node<T>>) {
    let priority = node.priority;
    if let Some(right_box) = std::mem::replace(&mut node.right, None) {
        assert!(right_box.value > node.value);
        if right_box.priority > priority {
            let mut left_box = std::mem::replace(node, right_box);
            left_box.right = std::mem::replace(&mut node.right, None);
            assert!(node.value > left_box.value);
            node.left = Some(left_box);
        } else {
            node.right = Some(right_box);
        }
    }
}

fn main() {
    let mut treap = Treap::new();
    treap.insert(1, 1);
    println!("{:?}", treap);
    treap.insert(2, 3);
    println!("{:?}", treap);
    treap.insert(3, 2);
    println!("{:?}", treap);
}
