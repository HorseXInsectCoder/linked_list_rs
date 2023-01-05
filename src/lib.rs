
use std::cell::{Ref, RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            elem,
            next: None,
            prev: None
        }))
    }
}

impl<T> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None
        }
    }

    fn push_front(&mut self, elem: T) {
        // let node = Rc::new(RefCell::new(Node {
        //     elem,
        //     next: None,
        //     prev: None
        // }));

        let node = Node::new(elem);

        match self.head.take() {
            Some(head) => {
                head.borrow_mut().prev = Some(node.clone());
                node.borrow_mut().next = Some(head);
                self.head = Some(node);
            },
            None => {
                self.tail = Some(node.clone());
                self.head = Some(node.clone());
            },
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node: Rc<RefCell<Node<T>>>| {
            // match node.borrow_mut().next.take() {
            //     Some(next) => {
            //         next.borrow_mut().prev = None;
            //         self.head = Some(next);
            //     },
            //     None => {
            //         self.tail.take();
            //     }
            // }

            if let Some(next) = node.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }

            Rc::try_unwrap(node).ok().unwrap().into_inner().elem
        })
    }

    fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node: &Rc<RefCell<Node<T>>>| {
            Ref::map(node.borrow(), |node| {
                &node.elem
            })
        })
    }

    fn push_back(&mut self, elem: T) {
        let node = Node::new(elem);
        if let Some(tail_node) = self.tail.take() {
            tail_node.borrow_mut().next = Some(node.clone());
            node.borrow_mut().prev = Some(tail_node);

            // 这一步是必须的，把指针指到最后一个元素
            self.tail = Some(node);
        } else {
            self.head = Some(node.clone());
            self.tail = Some(node);
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|node: Rc<RefCell<Node<T>>>| {
            if let Some(prev_node) = node.borrow_mut().prev.take() {
                prev_node.borrow_mut().next.take();
                self.tail = Some(prev_node);
            } else {
                self.head.take();
            }

            Rc::try_unwrap(node).ok().unwrap().into_inner().elem
        })
    }

    fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node: &Rc<RefCell<Node<T>>>| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_mut().map(|node: &mut Rc<RefCell<Node<T>>>| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_mut().map(|node: &mut Rc<RefCell<Node<T>>>| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics_work() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek_works() {
        let mut list = List::new();
        // assert_eq!(list.peek_front(), None);     binary operation `==` cannot be applied to type `Option<Ref<'_, _>>`
        assert!(list.peek_front().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(*list.peek_front().unwrap(), 3);
    }

    #[test]
    fn pop_back_works() {
        let mut list = List::new();
        assert_eq!(list.pop_back(), None);

        list.push_back(4);
        list.push_back(5);
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek_front_back_works() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(*list.peek_front().unwrap(), 3);
        assert_eq!(*list.peek_front_mut().unwrap(), 3);

        assert_eq!(*list.peek_back().unwrap(), 1);
        assert_eq!(*list.peek_back_mut().unwrap(), 1);
    }

}
