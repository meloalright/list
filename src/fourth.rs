use std::rc::Rc;
use std::cell::{Ref, RefCell};

pub struct List<T> {
    head: Link<T>, // 头指针
    tail: Link<T> // 尾指针
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Node {
                elem,
                next: None,
                prev: None
            }
        ))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: Link::None,
            tail: Link::None
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.head.take() {
            Some(orgin_head_node) => {
                orgin_head_node.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(orgin_head_node);
                self.head = Some(new_node);
            },
            None => {
                new_node.borrow_mut().next = None;
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(origin_head_node) => {
                match origin_head_node.borrow_mut().next.take() {
                    Some(next_node) => {
                        next_node.borrow_mut().prev.take();
                        self.head = Some(next_node);
                    },
                    None => {
                        self.tail.take();
                    }
                };
                Some(Rc::try_unwrap(origin_head_node).ok().unwrap().into_inner().elem)
            },
            None => {
                self.head = None;
                None
            }
        }
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::<i32>::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_peek() {
        let mut list = List::<i32>::new();
        assert!(list.peek_front().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(*list.peek_front().unwrap(), 3);
    }
}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {
            // nothing
        }
    }
}