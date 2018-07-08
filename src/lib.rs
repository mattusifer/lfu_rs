use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::Display;

pub mod nodes;
use nodes::{HasHead, Node};

#[derive(Debug)]
pub struct CacheNode<K, V>
where K: Hash + Eq + Clone {
    parent: Rc<RefCell<FrequencyList<K, V>>>,
    key: K,
    next: Option<Rc<RefCell<CacheNode<K, V>>>>,
    prev: Option<Weak<RefCell<CacheNode<K, V>>>>
}

impl<K, V> CacheNode<K, V>
where K: Hash + Eq + Clone {
   pub fn get_identifier(&self) -> (usize, usize) {
        let depth = if let Some(ref prev) = self.prev {
            self.get_depth_from_node(prev.upgrade().unwrap(), 1)
        } else {
            0
        };

        (self.parent.borrow().frequency, depth)
    }

    pub fn get_associated_data<'a>(&self, cache: &'a LFUCache<K, V>) -> &'a V {
        let id = self.get_identifier();

        for (data, node) in cache.cache.values() {
            if node.borrow().get_identifier() == id {
                return data;
            }
        }

        panic!("This node could not be found in the hashmap.")
    }
}

impl<K, V> nodes::Node for CacheNode<K, V>
where K: Hash + Eq + Clone {
    fn get_next(&self) -> Option<Rc<RefCell<CacheNode<K, V>>>> {
        match self.next {
            None => {None}
            Some(ref next) => {Some(Rc::clone(next))}
        }
    }
    fn set_next(&mut self, new_next: Option<Rc<RefCell<CacheNode<K, V>>>>) {
        self.next = new_next
    }
    fn get_prev(&self) -> Option<Weak<RefCell<CacheNode<K, V>>>> {
        match self.prev {
            None => {None}
            Some(ref prev) => {Some(Rc::downgrade(&Rc::clone(&prev.upgrade().unwrap())))}
        }
    }
    fn set_prev(&mut self, new_prev: Option<Weak<RefCell<CacheNode<K, V>>>>) {
        self.prev = new_prev
    }
}

#[derive(Debug)]
pub struct FrequencyList<K, V>
where K: Hash + Eq + Clone {
    head: Option<Rc<RefCell<CacheNode<K, V>>>>,
    frequency: usize,
    next: Option<Rc<RefCell<FrequencyList<K, V>>>>,
    prev: Option<Weak<RefCell<FrequencyList<K, V>>>>,
}

impl<K, V> FrequencyList<K, V>
where K: Hash + Eq + Clone,
      V: Display {
    pub fn new(freq: usize) -> Self {
        FrequencyList {
            head: None,
            frequency: freq,
            next: None, prev: None
        }
    }

    pub fn to_string(&self, cache: &LFUCache<K, V>) -> String
    where K: Hash + Eq + Clone, V: Display {
        self.reduce(|acc: String, node| {
            let mut acc = acc.clone();
            acc.push_str(&format!(" {}", node.borrow().get_associated_data(cache)));
            acc
        }, format!("Count {}:", self.frequency))
    }
}

impl<K, V> nodes::Node for FrequencyList<K, V>
where K: Hash + Eq + Clone {
    fn get_next(&self) -> Option<Rc<RefCell<FrequencyList<K, V>>>> {
        match self.next {
            None => {None}
            Some(ref next) => {Some(Rc::clone(next))}
        }
    }
    fn set_next(&mut self, new_next: Option<Rc<RefCell<FrequencyList<K, V>>>>) {
        self.next = new_next
    }
    fn get_prev(&self) -> Option<Weak<RefCell<FrequencyList<K, V>>>> {
        match self.prev {
            None => {None}
            Some(ref prev) => {Some(Rc::downgrade(&Rc::clone(&prev.upgrade().unwrap())))}
        }
    }
    fn set_prev(&mut self, new_prev: Option<Weak<RefCell<FrequencyList<K, V>>>>) {
        self.prev = new_prev
    }
}

impl<K, V> nodes::HasHead for FrequencyList<K, V>
where K: Hash + Eq + Clone {
    type Element = CacheNode<K, V>;

    fn get_head(&self) -> Option<Rc<RefCell<CacheNode<K, V>>>> {
        match self.head {
            None => {None}
            Some(ref head) => {Some(Rc::clone(head))}
        }
    }
    fn set_head(&mut self, new_head: Option<Rc<RefCell<CacheNode<K, V>>>>) {
        self.head = new_head
    }
}


#[derive(Debug)]
pub struct LFUCache<K, V>
where K: Hash + Eq + Clone {
    frequency_list_head: Option<Rc<RefCell<FrequencyList<K, V>>>>,
    cache: HashMap<K, (V, Rc<RefCell<CacheNode<K, V>>>)>,
    max_size: usize
}

impl<K, V> LFUCache<K, V>
where K: Hash + Eq + Clone, V: Display {
    pub fn new(max_size: usize) -> Self {
        LFUCache {
            frequency_list_head: None,
            cache: HashMap::new(),
            max_size: max_size
        }
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn to_string(&self) -> String {
        if self.frequency_list_head.is_none() {
            "<empty>".to_string()
        } else {
            self.reduce(|acc: String, list| {
                let mut acc = acc.clone();
                acc.push_str(&format!("{}\n", list.borrow().to_string(&self)));
                acc
            }, "".to_string())
        }
    }

    fn increment_node_frequency(&mut self, node: Rc<RefCell<CacheNode<K, V>>>) {
        // point the parent node ahead to the new parent
        let new_parent = {
            let node_i = node.borrow();

            let current_frequency = node_i.parent.borrow().frequency;
            let next_frequency_list = &node_i.parent.borrow().next;

            let there_is_a_gap = next_frequency_list.is_some() &&
                next_frequency_list.as_ref().unwrap().borrow().frequency != current_frequency + 1;

            // if either the next frequency does doesn't exist (this is
            // the last one) or the next frequency list's frequency is not
            // the current frequency + 1 (there is a gap), we need to
            // create a new frequency list and add this node to it.
            let new_parent = if next_frequency_list.is_none() || there_is_a_gap {
                // create new parent list
                Rc::new(RefCell::new(FrequencyList::new(current_frequency + 1)))
            } else {
                Rc::clone(next_frequency_list.as_ref().unwrap())
            };

            // if the next frequency list did exist, but there was a
            // gap, we need to point it back to the new parent
            if there_is_a_gap {
                next_frequency_list.as_ref().unwrap()
                    .borrow_mut().prev = Some(Rc::downgrade(&new_parent));
                new_parent.borrow_mut().next = Some(Rc::clone(
                    next_frequency_list.as_ref().unwrap()
                ));
            }

            new_parent
        };
        {
            // connect the new parent to the old parent
            let node_i = node.borrow();
            new_parent.borrow_mut().prev = Some(Rc::downgrade(&node_i.parent));
            node_i.parent.borrow_mut().next = Some(Rc::clone(&new_parent));
        }

        // if this is the last node in it's current parent, we
        // need to remove the current parent
        if node.borrow().next.is_none() && node.borrow().prev.is_none() {
            let node_i = node.borrow();

            // repoint the current parent's next and previous to each other
            node_i.parent.borrow().remove();

            // nullify all existing pointers
            node_i.parent.borrow_mut().next = None;
            node_i.parent.borrow_mut().prev = None;
        }

        // if this node is the head, we need to advance the head of
        // it's parent
        if node.borrow().prev.is_none() {
            let node_parent = Rc::clone(&node.borrow().parent);
            node_parent.borrow_mut().pop_head();
        }

        // set the parent as the new parent
        node.borrow_mut().parent = Rc::clone(&new_parent);

        // if this is the frequency list head, we need to repoint the head to the next list.
        if self.frequency_list_head.as_ref().unwrap().borrow().head.is_none() {
            self.frequency_list_head = Some(Rc::clone(&new_parent));
        }

        // point the node next back to the node previous, and vice versa
        node.borrow().remove();

        // officially add this node to it's new parent by prepending
        // it to the linked list
        new_parent.borrow_mut().push(Rc::clone(&node));
    }

    fn remove_node(&mut self, node: Rc<RefCell<CacheNode<K, V>>>) {
        if node.borrow().prev.is_none() {
            let node_parent = {
                Rc::clone(&node.borrow().parent)
            };
            if node.borrow().next.is_none() {
                node_parent.borrow_mut().remove();
            }
            node_parent.borrow_mut().pop_head();
        }

        node.borrow_mut().remove();
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let node = Rc::clone(&self.cache.get(key)?.1);
        self.increment_node_frequency(Rc::clone(&node));

        let (data, _) = self.cache.get(key)?;
        Some(&data)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let node = Rc::clone(&self.cache.get(&key)?.1);
        self.remove_node(Rc::clone(&node));

        let (data, _) = self.cache.remove(&key)?;
        Some(data)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if !self.cache.contains_key(&key) {
            // are there any frequency lists? (if not, this is the
            // first item in the cache.)g
            match self.pop_head() {
                None => {
                    // create a new list and a new node, connect them.
                    let mut new_frequency_list = Rc::new(RefCell::new(
                        FrequencyList::new(1)
                    ));
                    let new_node = Rc::new(RefCell::new(CacheNode {
                        parent: Rc::clone(&new_frequency_list),
                        key: key.clone(), next: None, prev: None
                    }));
                    new_frequency_list.borrow_mut().push(Rc::clone(&new_node));

                    // set the new list as the freq list head, insert
                    // the new node and value into the hashmap
                    self.push(new_frequency_list);
                    self.cache.insert(key, (value, Rc::clone(&new_node)));
                }
                Some(list) => {
                    // remove LFU item if we are over the max size
                    if self.len() >= self.max_size {
                        let key = {
                            let node_to_remove = &list.borrow().head;
                            let node_to_remove = Rc::clone(
                                &node_to_remove.as_ref().unwrap()
                            );
                            let node_to_remove = node_to_remove.borrow();
                            node_to_remove.key.clone()
                        };
                        self.remove(&key);
                    }

                    // if the existing list's frequency is 1, we can use it.
                    if list.borrow().frequency == 1 {
                        // create a new node and link it to the existing list
                        let new_node = Rc::new(RefCell::new(CacheNode {
                            parent: Rc::clone(&list),
                            key: key.clone(), next: None, prev: None
                        }));
                        list.borrow_mut().push(Rc::clone(&new_node));

                         // set the head back to the original list
                        self.push(Rc::clone(&list));

                        // insert the new data into the map
                        self.cache.insert(key, (value, Rc::clone(&new_node)));
                    } else {
                        // the existing list did not have a frequency
                        // of 1 - we can't use it.
                        let new_frequency_list = Rc::new(RefCell::new(
                            FrequencyList::new(1)
                        ));
                        let new_node = Rc::new(RefCell::new(CacheNode {
                            parent: Rc::clone(&new_frequency_list),
                            key: key.clone(), next: None, prev: None
                        }));

                        // mutable reference block
                        new_frequency_list.borrow_mut().push(Rc::clone(&new_node));
                        self.push(Rc::clone(&new_frequency_list));

                        // insert the new data into the map
                        self.cache.insert(key, (value, Rc::clone(&new_node)));
                    }
                }
            }

            None
        } else {
            let (old_value, node) = self.cache.remove(&key).unwrap();
            self.increment_node_frequency(Rc::clone(&node));
            self.cache.insert(key, (value, Rc::clone(&node)));
            Some(old_value)
        }
    }

}

impl<K, V> nodes::HasHead for LFUCache<K, V>
where K: Hash + Eq + Clone {
    type Element = FrequencyList<K, V>;

    fn get_head(&self) -> Option<Rc<RefCell<FrequencyList<K, V>>>> {
        match self.frequency_list_head {
            None => {None}
            Some(ref head) => {Some(Rc::clone(head))}
        }
    }
    fn set_head(&mut self, new_head: Option<Rc<RefCell<FrequencyList<K, V>>>>) {
        self.frequency_list_head = new_head
    }
}
