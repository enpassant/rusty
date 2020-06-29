extern crate alloc;

use alloc::sync::Arc;

#[derive(Debug)]
enum ListImpl<T> {
    Nil,
    Cons(T, Arc<ListImpl<T>>)
}

#[derive(Debug)]
pub struct List<T>(Arc<ListImpl<T>>);

impl<T: Copy> List<T> {
    pub fn add(&self, t: T) -> List<T> {
        List(Arc::new(ListImpl::Cons(t, self.0.clone())))
    }

    pub fn empty() -> List<T> {
        List(Arc::new(ListImpl::Nil))
    }

    pub fn fold<R, F>(&self, init: &R, f: F) -> R
    where
        R: Copy,
        F: FnOnce(&R, &T) -> R + Copy
    {
        let mut sum = *init;
        let mut arc = &self.0;
        loop {
            match arc.as_ref() {
                ListImpl::Cons(h, t) => {
                    arc = t;
                    sum = f(&sum, h);
                },
                ListImpl::Nil => break
            }
        }
        sum
    }

    pub fn head(&self) -> Option<T> {
         match self.0.as_ref() {
             ListImpl::Cons(h, _t) => Some(*h),
             ListImpl::Nil => None
         }
    }

    pub fn is_empty(&self) -> bool {
         match self.0.as_ref() {
             ListImpl::Cons(_h, _t) => false,
             ListImpl::Nil => true
         }
    }

    pub fn map<R, F: FnOnce(&T) -> R + Copy>(&self, f: F) -> List<R> {
         match self.0.as_ref() {
             ListImpl::Cons(h, t) => List(Arc::new(ListImpl::Cons(
                 f(h),
                 List::map(&List(t.clone()), f).0
             ))),
             ListImpl::Nil => List(Arc::new(ListImpl::Nil))
         }
    }

    pub fn new(elements: &[T]) -> List<T> {
        let mut list = List(Arc::new(ListImpl::Nil));
        for (_i, element) in elements.iter().rev().enumerate() {
            list = list.add(*element);
        }
        list
    }

    pub fn tail(&self) -> List<T> {
         match self.0.as_ref() {
             ListImpl::Cons(_h, t) => List(t.clone()),
             ListImpl::Nil => List(Arc::new(ListImpl::Nil))
         }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::string::String;
    use alloc::string::ToString;

    impl<T: Copy> List<T> {
        fn map_strong_count(&self) -> String {
            let mut sc = Arc::strong_count(&self.0).to_string();
            let mut arc = &self.0;
            loop {
                match arc.as_ref() {
                    ListImpl::Cons(_h, t) => {
                        arc = t;
                        sc.push_str(", ");
                        sc.push_str(Arc::strong_count(arc).to_string().as_str());
                    },
                    ListImpl::Nil => break
                }
            }
            sc
        }
    }

    #[test]
    fn test_empty() {
        let empty_list: List<i32> = List::empty();

        assert!(empty_list.head().is_none());
        assert!(empty_list.is_empty());
    }

    #[test]
    fn test_add() {
        let empty_list: List<i32> = List::empty();
        let one_item_list = empty_list.add(17);
        assert!(!one_item_list.is_empty());
        assert_eq!(one_item_list.head(), Some(17));
    }

    #[test]
    fn test_fold() {
        let list: List<i32> = List::new(&[1, 2, 3, 4]);
        let sum = list.fold(&0, |x, y| x + y);
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_map() {
        let list: List<i32> = List::new(&[1, 2, 3, 4]);
        let list_double = list.map(|x| x * 2);
        assert_eq!(list_double.head(), Some(2));
        assert_eq!(list_double.tail().head(), Some(4));
        assert_eq!(list_double.tail().tail().head(), Some(6));
        assert_eq!(list_double.tail().tail().tail().head(), Some(8));
    }

    #[test]
    fn test_context() {
        let list: List<i32> = List::new(&[1, 2, 3, 4]);
        let count_list = list.map_strong_count();
        assert_eq!(count_list, "1, 1, 1, 1, 1");

        {
            let lt = list.tail();
            let cl_lt = lt.map_strong_count();
            assert_eq!(cl_lt, "2, 1, 1, 1");
            let count_list = list.map_strong_count();
            assert_eq!(count_list, "1, 2, 1, 1, 1");
            {
                let lm = list.map(|x| x * 2);
                let cl_lm = lm.map_strong_count();
                assert_eq!(cl_lm, "1, 1, 1, 1, 1");
                let count_list = list.map_strong_count();
                assert_eq!(count_list, "1, 2, 1, 1, 1");
            }
            let count_list = list.map_strong_count();
            assert_eq!(count_list, "1, 2, 1, 1, 1");

            let ltt = lt.tail();
            let cl_ltt = ltt.map_strong_count();
            assert_eq!(cl_ltt, "2, 1, 1");
            let count_list = list.map_strong_count();
            assert_eq!(count_list, "1, 2, 2, 1, 1");

            {
                let ltta = ltt.add(3);
                let cl_ltta = ltta.map_strong_count();
                assert_eq!(cl_ltta, "1, 3, 1, 1");
                let count_list = list.map_strong_count();
                assert_eq!(count_list, "1, 2, 3, 1, 1");
            }

            let cl_ltt = ltt.map_strong_count();
            assert_eq!(cl_ltt, "2, 1, 1");
            let count_list = list.map_strong_count();
            assert_eq!(count_list, "1, 2, 2, 1, 1");
        }

        let count_list = list.map_strong_count();
        assert_eq!(count_list, "1, 1, 1, 1, 1");
    }
}

