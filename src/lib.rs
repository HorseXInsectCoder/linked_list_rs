

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

pub struct List<T> {
    left: Stack<T>,
    right: Stack<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            left: Stack::new(),
            right: Stack::new(),
        }
    }

    pub fn push_left(&mut self, elem: T) {
        self.left.push(elem);
    }

    pub fn push_right(&mut self, elem: T) {
        self.right.push(elem);
    }

    pub fn pop_left(&mut self) -> Option<T> {
        self.left.pop()
    }

    pub fn pop_right(&mut self) -> Option<T> {
        self.right.pop()
    }

    pub fn peek_left(&self) -> Option<&T> {
        self.left.peek()
    }

    pub fn peek_right(&self) -> Option<&T> {
        self.right.peek()
    }

    pub fn peek_left_mut(&mut self) -> Option<&mut T> {
        self.left.peek_mut()
    }

    pub fn peek_right_mut(&mut self) -> Option<&mut T> {
        self.right.peek_mut()
    }

    // 左边的元素出栈，push到右边的栈
    pub fn go_left(&mut self) -> bool {
        self.left.pop_node().map(|node| {
            self.right.push_node(node);
        }).is_some()
    }

    pub fn go_right(&mut self) -> bool {
        self.right.pop_node().map(|node| {
            self.left.push_node(node);
        }).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics_work() {
        let mut list: List<i32> = List::new();

        list.push_left(0);                      // [0 |   ]
        list.push_right(1);                     // [0 | 1]
        assert_eq!(list.peek_left(), Some(&0));
        assert_eq!(list.peek_right(), Some(&1));

        list.push_left(2);                      // [0, 2 | 1]
        list.push_left(3);                      // [0, 2, 3 | 1]
        list.push_right(4);                     // [0, 2, 3 | 4, 1]

        while list.go_left() {} // 把左栈的所有元素都移到右栈

        assert_eq!(list.pop_left(), None);          // [ | 0, 2, 3, 4, 1]
        assert_eq!(list.pop_right(), Some(0));      // [ | 2, 3, 4, 1]
        assert_eq!(list.pop_right(), Some(2));      // [ | 3, 4, 1]

        list.push_left(5);                     // [5 | 3, 4, 1]
        assert_eq!(list.pop_right(), Some(3));      // [5 | 4, 1]
        assert_eq!(list.pop_left(), Some(5));       // [  | 4, 1]
        assert_eq!(list.pop_right(), Some(4));      // [  | 1]
        assert_eq!(list.pop_right(), Some(1));      // [     ]
    }
}
