#[allow(unused_imports)] // but why?
use higher::Functor;
use higher_derive::{Functor, Lift};

#[derive(Lift, Functor, PartialEq, Debug)]
pub struct NamedStruct<A> {
    value: A,
    counter: usize,
}

#[test]
fn derive_functor_named_struct() {
    let a = NamedStruct {
        value: 1,
        counter: 2,
    };
    let b = a.map(|a| a.to_string());
    assert_eq!(
        NamedStruct {
            value: "1".to_string(),
            counter: 2
        },
        b
    );
}

#[derive(Lift, Functor, PartialEq, Debug)]
pub struct UnnamedStruct<A>(usize, A);

#[test]
fn derive_functor_unnamed_struct() {
    let a = UnnamedStruct(1, 2);
    let b = a.map(|a| a.to_string());
    assert_eq!(UnnamedStruct(1, "2".to_string()), b);
}

#[derive(Lift, Functor, PartialEq, Debug)]
pub enum Maybe<A> {
    Definitely(usize, A),
    SortOf { counter: usize, value: A },
    NotReally,
}

#[test]
fn simple_derive() {
    let maybe_int: Maybe<u8> = Maybe::Definitely(1, 2);
    let maybe_str = maybe_int.map(|i| i.to_string());
    assert_eq!(Maybe::Definitely(1, "2".to_string()), maybe_str);
    let maybe_int: Maybe<u8> = Maybe::SortOf {
        counter: 1,
        value: 2,
    };
    let maybe_str = maybe_int.map(|i| i.to_string());
    assert_eq!(
        Maybe::SortOf {
            counter: 1,
            value: "2".to_string()
        },
        maybe_str
    );
}

#[derive(Lift, Functor, PartialEq, Debug)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

#[test]
fn derive_with_extra_vars() {
    let either_int: Either<u8, ()> = Either::Left(1);
    let either_str = either_int.map(|i| i.to_string());
    assert_eq!(Either::Left("1".to_string()), either_str);
}
