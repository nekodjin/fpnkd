use std::fmt;
use std::ops;
use std::sync;

use sync::Arc;

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

    pub fn tail(self) -> Self {
        Self {
            head: self.head.unwrap().next.clone(),
        }
    }

    pub fn prepend(self, val: T) -> Self {
        self.prepend_shared(Arc::new(val))
    }

    pub fn prepend_shared(self, val: Arc<T>) -> Self {
        Self {
            head: Some(Arc::new(Node {
                val,
                next: self.head,
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

            last = &mut Arc::get_mut(last.as_mut().unwrap()).unwrap().next;
        }

        *last = Some(ls.head.clone().unwrap());

        list
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
        Self { head: cons(source) }
    }
}

fn cons<I, T>(source: I) -> Option<Arc<Node<T>>>
where
    I: IntoIterator<Item = Arc<T>>,
{
    let mut iter = source.into_iter();

    Some(Arc::new(Node {
        val: iter.next()?,
        next: cons(iter),
    }))
}
