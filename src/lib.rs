

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

struct List<T> {
    head: Link<T>
}

impl<T> List<T> {
    fn new() -> Self {
        Self {
            head: None
        }
    }

    fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem,
            next: self.head.take()
        });
        self.head = Some(node);
    }

    fn pop(&mut self) -> Option<T> {
        // match self.head.take() {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }

        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // 取链尾元素的值的引用，返回的是一个不可变引用
    fn peek(&self) -> Option<&T> {
        // Converts from &Option<T> to Option<&T>.
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        // Converts from &mut Option<T> to Option<&mut T>
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
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

        list.push("hello");
        list.push("world");
        list.push("swiss");
        assert_eq!(list.pop(), Some("swiss"));
        assert_eq!(list.pop(), Some("world"));
        assert_eq!(list.pop(), Some("hello"));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek_works() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push("hello");
        list.push("world");
        list.push("swiss");

        assert_eq!(list.peek(), Some(&"swiss"));
        assert_eq!(list.peek_mut(), Some(&mut "swiss"));

        list.peek_mut().map(|value| {
            *value = "switzerland";
        });
        assert_eq!(list.peek(), Some(&"switzerland"));
        assert_eq!(list.pop(), Some("switzerland"));
        assert_eq!(list.pop(), Some("world"));
    }
}
