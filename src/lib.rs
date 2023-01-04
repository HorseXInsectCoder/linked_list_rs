use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        Self {
            head: None
        }
    }

    fn append(&mut self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                // 由于使用了Rc，所以这里是clone，而不是之前的take。即这里的引用计数加 1
                next: self.head.clone(),
            }))
        }
    }

    fn tail(&self) -> List<T> {
        List {
            // Returns [`None`] if the option is [`None`], otherwise calls `f` with the wrapped value and returns the result
            // and_then的参数是一个闭包
            head: self.head.as_ref().and_then(|node| {
                node.next.clone()
            })
        }
    }

    // 返回头节点元素
    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<T> List<T> {
    fn iter(&self) -> Iter<T> {
        Iter {
            // next: self.head.as_ref()     // 会得到 Option<&Rc<Node<T>>>

            // Converts from Option<T> (or &Option<T>) to Option<&T::Target>.
            next: self.head.as_deref()     // Option<&<T as Deref>::Target>

        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
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
