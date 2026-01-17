use super::*;

#[test]
fn consistent_str_hash_key() {
    let hash = HashKey::new_str("foobarbaz");
    assert_eq!(4247283670897481861, hash.0);
}

#[test]
fn consistent_slice_hash_key() {
    let hash = HashKey::new_slice(&[3, 1, 4, 1, 5, 9, 2, 6, 5, 4]);
    assert_eq!(8947523901814331430, hash.0);
}

#[test]
fn intern_strings() {
    let mut interner = StrInterner::default();

    let foo = interner.intern("foo");
    let empty = interner.intern("");
    let bar = interner.intern("bar");

    assert_eq!("foo", &interner[foo]);
    assert_eq!("", &interner[empty]);
    assert_eq!("bar", &interner[bar]);
}

#[test]
fn intern_vec() {
    let mut interner = Interner::<u32>::default();

    let range = interner.intern(vec![1, 2, 3]);
    assert_eq!(&[1, 2, 3], &interner[range]);

    let range = interner.intern(vec![4, 5, 6]);
    assert_eq!(&[4, 5, 6], &interner[range]);
}

#[test]
fn intern_vec_and_array() {
    let mut interner = Interner::<u32>::default();

    let range = interner.intern(vec![1, 2, 3]);
    assert_eq!(&[1, 2, 3], &interner[range]);

    let range2 = interner.intern_slice(&[1, 2, 3]);

    assert_eq!(range, range2);
}

#[test]
fn interning_duplicate_strings() {
    let mut interner = StrInterner::default();

    let foo = interner.intern("foo");
    let foo2 = interner.intern("foo");

    assert_eq!(foo, foo2);
}

#[test]
fn interning_duplicate_slices() {
    let mut interner = Interner::<u32>::default();

    let foo = interner.intern(vec![1, 2, 3]);
    let foo2 = interner.intern(vec![1, 2, 3]);

    assert_eq!(foo, foo2);
}

#[test]
fn intern_and_get_str() {
    let mut interner = StrInterner::default();

    let key = interner.intern("foo");

    assert_eq!(Some(key), interner.get("foo"));
}

#[test]
fn intern_and_get_slice() {
    let mut interner = Interner::default();

    let key = interner.intern_slice(&[1, 2, 3, 4]);

    assert_eq!(Some(key), interner.get(&[1, 2, 3, 4]));
}
