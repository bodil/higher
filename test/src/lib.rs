#[allow(unused_imports)] // but why?
use higher::{Bifunctor, Bilift, Functor};
use higher_derive::{Bilift, Functor, Lift};

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

#[derive(Lift, Bilift, Functor, PartialEq, Debug)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B, C, D> Bifunctor<A, B, C, D> for Either<A, B> {
    fn bimap<L, R>(self, left: L, right: R) -> <Self as Bilift<A, B, C, D>>::Target
    where
        L: Fn(A) -> C,
        R: Fn(B) -> D,
    {
        match self {
            Either::Left(a) => Either::Left(left(a)),
            Either::Right(b) => Either::Right(right(b)),
        }
    }
}

#[test]
fn derive_with_extra_vars() {
    let either_int: Either<u8, ()> = Either::Left(1);
    let either_str = either_int.map(|i| i.to_string());
    assert_eq!(Either::Left("1".to_string()), either_str);

    let either_left: Either<u8, u8> = Either::Left(1);
    assert_eq!(
        Either::Left("1".to_string()),
        either_left.bimap(|i| i.to_string(), |i| i.to_string())
    );
    let either_right: Either<u8, u8> = Either::Right(1);
    assert_eq!(
        Either::Right("1".to_string()),
        either_right.bimap(|i| i.to_string(), |i| i.to_string())
    );
}
