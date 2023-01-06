

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

struct Stack<T> {
    head: Link<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    fn push_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head.take();
        self.head = Some(node);
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem,
            next: None,
        });
        self.push_node(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|node| {
            node.elem
        })
    }

    fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.take().map(|mut node: Box<Node<T>>| {
            self.head = node.next.take();
            node
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics_work() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
    }

    #[test]
    fn into_iter_works() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        // 注：这里的push是从尾部push的
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_works() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        // 注：这里的push是从尾部push的
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut_works() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        // 注：这里的push是从尾部push的
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
