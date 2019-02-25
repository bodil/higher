pub trait Lift<A, B> {
    type Target1;
}

pub trait Lift3<A, B, C>: Lift<A, C> {
    type Target2;
    fn cast(
        mut from: <<Self as Lift3<A, B, C>>::Target2 as Lift<B, C>>::Target1,
    ) -> <Self as Lift<A, C>>::Target1
    where
        <Self as Lift3<A, B, C>>::Target2: Lift<B, C>,
    {
        unsafe {
            let ptr = &mut from as *mut _ as *mut <Self as Lift<A, C>>::Target1;
            ::std::ptr::read(ptr)
        }
    }
}

impl<A, B> Lift<A, B> for Option<A> {
    type Target1 = Option<B>;
}

impl<A, B, E> Lift<A, B> for Result<A, E> {
    type Target1 = Result<B, E>;
}

impl<A, B> Lift<A, B> for Vec<A> {
    type Target1 = Vec<B>;
}

impl<A, B, C> Lift3<A, B, C> for Option<A> {
    type Target2 = Option<B>;
}

impl<A, B, C, E> Lift3<A, B, C> for Result<A, E> {
    type Target2 = Result<B, E>;
}

impl<A, B, C> Lift3<A, B, C> for Vec<A> {
    type Target2 = Vec<B>;
}
