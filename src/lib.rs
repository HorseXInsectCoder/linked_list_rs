
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

// 要实现的迭代器
// IntoIter => T
// Iter => &T
// IterMut => &mut T
struct IntoIter<T>(List<T>);

impl<T> List<T> {
    // 这里的self是List<T>
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    // 这里的self是IntoIter<T>
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    fn iter(&self) -> Iter<T> {     // 这里省略了生命周期
        Iter{
            // 由于我们是Option<&'a Node<T>>,引用在Option里面，所以要用as_deref，把self.head拿到的数据转为Option<&'a Node<T>>
            // Converts from Option<T> (or &Option<T>) to Option<&T::Target>.
            next: self.head.as_deref()
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem      // 返回引用
        })
    }
}

struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<T> List<T> {
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut()
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
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

    #[test]
    fn into_iter_works() {
        let mut list = List::new();

        list.push("hello");
        list.push("world");
        list.push("swiss");

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some("swiss"));
        assert_eq!(iter.next(), Some("world"));
        assert_eq!(iter.next(), Some("hello"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_works() {
        let mut list = List::new();

        list.push("hello");
        list.push("world");
        list.push("swiss");

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&"swiss"));
        assert_eq!(iter.next(), Some(&"world"));
        assert_eq!(iter.next(), Some(&"hello"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut_works() {
        let mut list = List::new();

        list.push("hello");
        list.push("world");
        list.push("swiss");

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut "swiss"));
        assert_eq!(iter.next(), Some(&mut "world"));
        assert_eq!(iter.next(), Some(&mut "hello"));
        assert_eq!(iter.next(), None);
    }
}
