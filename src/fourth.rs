use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};

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

    pub fn push_back(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.tail.take() {
            Some(orgin_tail_node) => {
                orgin_tail_node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(orgin_tail_node);
                self.tail = Some(new_node);
            },
            None => {
                new_node.borrow_mut().prev = None;
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

    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(origin_tail_node) => {
                match origin_tail_node.borrow_mut().prev.take() {
                    Some(prev_node) => {
                        prev_node.borrow_mut().next.take();
                        self.tail = Some(prev_node);
                    },
                    None => {
                        self.head.take();
                    }
                };
                Some(Rc::try_unwrap(origin_tail_node).ok().unwrap().into_inner().elem)
            },
            None => {
                self.tail = None;
                None
            }
        }
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

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

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
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

    #[test]
    fn test_push_and_pop_back() {
        let mut list = List::<i32>::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_back(-1);
        list.push_back(-2);
        list.push_back(-3);

        assert_eq!(*list.peek_front().unwrap(), 3);
        assert_eq!(*list.peek_front_mut().unwrap(), 3);
        assert_eq!(*list.peek_front_mut().unwrap(), 3);
        assert_eq!(*list.peek_back().unwrap(), -3);
        assert_eq!(list.pop_back(), Some(-3));
        assert_eq!(list.pop_back(), Some(-2));
        assert_eq!(list.pop_back(), Some(-1));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), None);
    }


    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1); list.push_front(2); list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }
}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {
            // nothing
        }
    }
}