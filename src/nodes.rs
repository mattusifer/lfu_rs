use std::rc::{Rc, Weak};
use std::cell::RefCell;

// Trait for objects that contain a reference to the head of a linked list
pub trait HasHead
where Self: Sized {
    type Element: Node;
    fn get_head(&self) -> Option<Rc<RefCell<Self::Element>>>;
    fn set_head(&mut self, new_head: Option<Rc<RefCell<Self::Element>>>);

    fn push(&mut self, node: Rc<RefCell<Self::Element>>) {
        match self.get_head() {
            None => {
                node.borrow_mut().set_next(None);
                node.borrow_mut().set_prev(None);
                self.set_head(Some(node));
            }
            Some(ref head) => {
                head.borrow_mut().set_prev(Some(Rc::downgrade(&node)));
                node.borrow_mut().set_next(Some(Rc::clone(head)));
                node.borrow_mut().set_prev(None);
                self.set_head(Some(node));
            }
        }
    }

    fn pop_head(&mut self) -> Option<Rc<RefCell<Self::Element>>> {
        if self.get_head().is_none() {None}
        else {
            let head = self.get_head().unwrap();

            match head.borrow().get_next() {
                None => {
                    self.set_head(None);
                }
                Some(ref new_head) => {
                    new_head.borrow_mut().set_prev(None);
                    self.set_head(Some(Rc::clone(new_head)));
                }
            }

            head.borrow_mut().set_next(None);
            Some(Rc::clone(&head))
        }
    }

    fn reduce<U, F>(&self, f: F, initial: U) -> U
    where F: Fn(U, Rc<RefCell<Self::Element>>) -> U {
        match self.get_head() {
            None => {initial}
            Some(ref head) => {
                let out = f(initial, Rc::clone(head));
                head.borrow().reduce_forward(f, out)
            }
        }
    }
}

// Trait for objects that are nodes in a linked list
pub trait Node
where Self: Sized {

    fn get_next(&self) -> Option<Rc<RefCell<Self>>>;
    fn set_next(&mut self, new_next: Option<Rc<RefCell<Self>>>);
    fn get_prev(&self) -> Option<Weak<RefCell<Self>>>;
    fn set_prev(&mut self, new_prev: Option<Weak<RefCell<Self>>>);

    fn is_head(&self) -> bool {
        self.get_prev().is_none()
    }

    fn is_only_child(&self) -> bool {
        self.get_prev().is_none() && self.get_next().is_none()
    }

    // Fold left from this node
    fn reduce_forward<U, F>(&self, f: F, out: U) -> U
    where F: Fn(U, Rc<RefCell<Self>>) -> U {
        match self.get_next() {
            None => {out}
            Some(ref next) => {
                let out = f(out, Rc::clone(next));
                next.borrow().reduce_forward(f, out)
            }
        }
    }

    fn remove(&mut self) {
        match self.get_next() {
            None => {}
            Some(ref next) => {
                match self.get_prev() {
                    None => {next.borrow_mut().set_prev(None);}
                    Some(ref prev) => {
                        next.borrow_mut().set_prev(Some(
                            Rc::downgrade(&Rc::clone(&prev.upgrade().unwrap()))
                        ));
                    }
                }
            }
        }
        match self.get_prev() {
            None => {}
            Some(ref prev) => {
                let prev = prev.upgrade().unwrap();
                match self.get_next() {
                    None => {prev.borrow_mut().set_next(None);}
                    Some(ref next) => {
                        prev.borrow_mut().set_next(Some(Rc::clone(next)));
                    }
                }
            }
        }
        self.set_next(None);
        self.set_prev(None);
    }

    // Get the depth from this node to the list head
    fn get_depth_from_node(&self, node: Rc<RefCell<Self>>, depth: usize) -> usize {
        match node.borrow().get_prev() {
            Some(ref prev) => {
                self.get_depth_from_node(prev.upgrade().unwrap(), depth + 1)
            }
            None => {
                depth
            }
        }
    }
}
