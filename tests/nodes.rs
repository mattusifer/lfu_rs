extern crate lfu_rs;

use lfu_rs::nodes::{HasHead, Node};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct TestHead {
    head: Option<Rc<RefCell<TestNode>>>
}

impl TestHead {
    fn new() -> Self {
        let mut test_head = TestHead {
            head: Some(Rc::new(RefCell::new(TestNode::new("node3"))))
        };
        test_head.push(Rc::new(RefCell::new(TestNode::new("node2"))));
        test_head.push(Rc::new(RefCell::new(TestNode::new("node1"))));
        test_head
    }

    fn to_string(&self) -> String {
        self.reduce(|acc: String, node| {
            let mut acc = acc.clone();
            acc.push_str(&format!(" {}", node.borrow().id));
            acc
        }, "Nodes:".to_string())
    }
}

impl HasHead for TestHead {
    type Element = TestNode;

    fn get_head(&self) -> Option<Rc<RefCell<TestNode>>> {
        match self.head {
            None => {None},
            Some(ref head) => {
                Some(Rc::clone(head))
            }
        }
    }
    fn set_head(&mut self, new_head: Option<Rc<RefCell<TestNode>>>) {
        self.head = new_head
    }
}

#[derive(Debug)]
struct TestNode {
    id: String,
    next: Option<Rc<RefCell<TestNode>>>,
    prev: Option<Weak<RefCell<TestNode>>>
}

impl TestNode {
    fn new(id: &str) -> Self {
        TestNode {
            id: id.to_string(),
            next: None, prev: None
        }
    }
}

impl Node for TestNode {
    fn get_next(&self) -> Option<Rc<RefCell<TestNode>>> {
        match self.next {
            None => {None}
            Some(ref next) => {Some(Rc::clone(next))}
        }
    }
    fn set_next(&mut self, new_next: Option<Rc<RefCell<TestNode>>>) {
        self.next = new_next
    }
    fn get_prev(&self) -> Option<Weak<RefCell<TestNode>>> {
        match self.prev {
            None => {None}
            Some(ref prev) => {Some(Rc::downgrade(&Rc::clone(&prev.upgrade().unwrap())))}
        }
    }
    fn set_prev(&mut self, new_prev: Option<Weak<RefCell<TestNode>>>) {
        self.prev = new_prev
    }
}

#[test]
fn test_structures() {
    let head = TestHead::new();
    assert_eq!(head.to_string(), "Nodes: node1 node2 node3")
}

#[test]
fn pop_head() {
    let mut test_head = TestHead::new();

    let old_head = test_head.pop_head();
    let old_head = old_head.as_ref().unwrap().borrow();

    let new_head = test_head.head;
    let new_head = new_head.as_ref().unwrap().borrow();

    assert_eq!(old_head.id, "node1");
    assert_eq!(new_head.id, "node2");

    assert!(old_head.next.is_none());
    assert!(new_head.prev.is_none());
}

#[test]
fn remove() {
    let test_head = TestHead::new();
    let node_to_remove = {
        let head = test_head.get_head();
        let head = head.as_ref().unwrap().borrow();

        let node_to_remove = &head.next;
        Rc::clone(node_to_remove.as_ref().unwrap())
    };

    node_to_remove.borrow_mut().remove();

    assert_eq!(test_head.to_string(), "Nodes: node1 node3");
}
