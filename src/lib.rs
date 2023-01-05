use std::cell::RefCell;
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
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics_work() {
        let mut list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append("hello").append("world").append("swiss");
        assert_eq!(list.head(), Some(&"swiss"));

        let list = list.tail();
        assert_eq!(list.head(), Some(&"world"));

        let list = list.tail().tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter_works() {
        let mut list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
