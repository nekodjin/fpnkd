use std::mem;
use std::sync;

use crate::list::List;

use sync::Arc;

#[test]
fn to_from_iter_works() {
    let expected = vec![Arc::new(1), Arc::new(2), Arc::new(3)];

    let found: Vec<_> = List::from([1, 2, 3]).collect();

    assert_eq!(expected, found);
}

#[test]
fn append_works() {
    let l1 = List::from([1, 2, 3]);
    let l2 = List::from([4, 5, 6]);
    let l3 = l1.append(l2);

    let expected = List::from([1, 2, 3, 4, 5, 6]);

    assert_eq!(expected, l3);
}

#[test]
fn prepend_works() {
    let list = List::from([2, 3, 4, 5]);
    let list = list.prepend(1);

    let expected = List::from([1, 2, 3, 4, 5]);

    assert_eq!(expected, list);
}

#[test]
fn head_works() {
    let list = List::from([1, 2, 3]);
    assert_eq!(1, *list.head());
}

#[test]
fn tail_works() {
    let l1 = List::from([1, 2, 3]);
    let l2 = List::from([0, 1, 2, 3]).tail();

    assert_eq!(l1, l2);
}

#[test]
fn eq_works() {
    let l1 = List::from([1, 2, 3]);
    let l2 = List::from([1, 2, 3]);

    assert_eq!(l1, l2);
}

#[test]
fn ne_works_value() {
    let l1 = List::from([1, 2, 3]);
    let l2 = List::from([4, 5, 6]);

    assert_ne!(l1, l2);
    assert_ne!(l2, l1);
}

#[test]
fn ne_works_length() {
    let l1 = List::from([1, 2, 3]);
    let l2 = List::from([1, 2, 3, 4, 5]);

    assert_ne!(l1, l2);
    assert_ne!(l2, l1);
}

#[test]
fn index_works() {
    let list = List::from([1, 2, 3]);

    assert_eq!(1, *list[0]);
    assert_eq!(2, *list[1]);
    assert_eq!(3, *list[2]);
}

#[test]
#[should_panic]
fn index_out_of_bounds() {
    let list = List::from([1, 2, 3]);
    let val = list[20].clone();

    mem::drop(val);
}

#[test]
fn no_stack_overflow() {
    mem::drop(List::from([1]).cycle().take(5_000_000));
}
