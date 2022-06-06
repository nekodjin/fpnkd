use fpnkd::list::List;

fn main() {
    let list = List::from([1, 4, 2, 7, 4, 8, 9, 3]);
    let sorted = sort(list);

    println!("{sorted:?}");
}

fn sort<T>(ls: List<T>) -> List<T>
where
    T: PartialOrd,
{
    if ls.len() < 2 {
        return ls;
    }

    let first = ls.clone().take(ls.len() / 2).collect();
    let last = ls.clone().skip(ls.len() / 2).collect();

    merge(sort(first), sort(last))
}

fn merge<T>(xs: List<T>, ys: List<T>) -> List<T>
where
    T: PartialOrd,
{
    if xs.is_empty() {
        return ys;
    }

    if ys.is_empty() {
        return xs;
    }

    let x = xs.head();
    let y = ys.head();

    if x < y {
        merge(xs.tail(), ys).prepend_shared(x)
    } else {
        merge(xs, ys.tail()).prepend_shared(y)
    }
}
