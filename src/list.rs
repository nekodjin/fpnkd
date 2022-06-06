use std::fmt;
use std::ops;
use std::sync;

use sync::Arc;

// TODO
// use custom allocator once
// Allocator API is stabilized
pub struct List<T> {
    head: Option<Arc<Node<T>>>,
}

struct Node<T> {
    val: Arc<T>,
    next: Option<Arc<Self>>,
}

impl<T> List<T> {
    pub const fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.clone().count()
    }

    pub fn head(&self) -> Arc<T> {
        self.head.as_ref().unwrap().val.clone()
    }

    pub fn tail(mut self) -> Self {
        Self {
            head: self.head.take().unwrap().next.clone(),
        }
    }

    pub fn prepend(self, val: T) -> Self {
        self.prepend_shared(Arc::new(val))
    }

    pub fn prepend_shared(mut self, val: Arc<T>) -> Self {
        Self {
            head: Some(Arc::new(Node {
                val,
                next: self.head.take(),
            })),
        }
    }

    pub fn append(self, ls: Self) -> Self {
        if self.is_empty() {
            return ls;
        }

        if ls.is_empty() {
            return self;
        }

        let mut list = Self::new();
        let mut last = &mut list.head;

        for element in self {
            let node = Node {
                val: element,
                next: None,
            };

            *last = Some(Arc::new(node));
            last = unsafe {
                &mut last
                    .as_mut()
                    // TODO
                    // replace with get_mut_unchecked
                    // once it is stabilized
                    .map(Arc::get_mut)
                    .unwrap_unchecked()
                    .unwrap_unchecked()
                    .next
            };
        }

        *last = Some(ls.head.clone().unwrap());

        list
    }
}

impl<T> List<T>
where
    T: Clone,
{
    pub fn deep_clone(&self) -> Self {
        self.clone().map(|x| (*x).clone()).collect()
    }
}

impl<T> fmt::Debug for List<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        Self {
            head: self.head.clone(),
        }
    }
}

impl<T, U> PartialEq<List<U>> for List<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, rhs: &List<U>) -> bool {
        let mut lhs = self.clone();
        let mut rhs = rhs.clone();

        loop {
            let l = lhs.next();
            let r = rhs.next();

            if l.is_none() && r.is_none() {
                return true;
            }

            if l.is_none() || r.is_none() {
                return false;
            }

            let l = l.unwrap();
            let r = r.unwrap();

            if *l != *r {
                return false;
            }
        }
    }
}

impl<T> Eq for List<T> where T: Eq {}

impl<T> ops::Index<usize> for List<T> {
    type Output = Arc<T>;

    fn index(&self, mut idx: usize) -> &Arc<T> {
        let mut node = self.head.as_ref().unwrap();

        while idx != 0 {
            node = node.next.as_ref().unwrap();
            idx -= 1;
        }

        &node.val
    }
}

impl<T> Iterator for List<T> {
    type Item = Arc<T>;

    fn next(&mut self) -> Option<Arc<T>> {
        let head = self.head.as_mut()?;
        let val = head.val.clone();
        self.head = head.next.clone();

        Some(val)
    }
}

impl<I, T> From<I> for List<T>
where
    I: IntoIterator<Item = T>,
{
    fn from(source: I) -> Self {
        source.into_iter().collect()
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I>(source: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        source.into_iter().map(Arc::new).collect()
    }
}

impl<T> FromIterator<Arc<T>> for List<T> {
    fn from_iter<I>(source: I) -> Self
    where
        I: IntoIterator<Item = Arc<T>>,
    {
        let mut list = Self::new();
        let mut last = &mut list.head;

        for elem in source {
            let node = Node {
                val: elem,
                next: None,
            };

            *last = Arc::new(node).into();
            last = unsafe {
                &mut last
                    .as_mut()
                    // TODO
                    // replace with get_mut_unchecked
                    // once it is stabilized
                    .map(Arc::get_mut)
                    .unwrap_unchecked()
                    .unwrap_unchecked()
                    .next
            };
        }

        list
    }
}

// custom drop prevents stack overflow
// when freeing long lists
impl<T> ops::Drop for Node<T> {
    fn drop(&mut self) {
        let mut temp = self.next.take();

        while let Some(mut curr) = temp {
            temp = match Arc::get_mut(&mut curr) {
                Some(curr) => curr.next.take(),
                None => return,
            };
        }
    }
}
