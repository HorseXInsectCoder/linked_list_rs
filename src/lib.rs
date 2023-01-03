use std::mem::replace;

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        Self {
            head: Link::Empty
        }
    }

    pub fn push(&mut self, elem: i32) {
        let node = Box::new(Node {
            elem,
            // 新的节点指向原来头节点所指的指针；做完这一步后，头节点是指向空的
            next: replace(&mut self.head, Link::Empty),
        });
        // 头节点指向新的节点
        self.head = Link::More(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // 循环把元素都drop掉
        let mut link = replace(&mut self.head, Link::Empty);
        while let Link::More(mut node) = link {
            link = replace(&mut node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
