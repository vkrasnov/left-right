/// Types that implement this trait can be cheaply copied by (potentially) aliasing the data they
/// contain.
pub trait ShallowCopy {
    /// Perform an aliasing copy of this value.
    ///
    /// The use of this method is *only* safe if the values involved are never mutated, and only
    /// one of the copies is dropped; the remaining copies must be forgotten with `mem::forget`.
    unsafe fn shallow_copy(&mut self) -> Self;
}

use std::sync::Arc;
impl<T> ShallowCopy for Arc<T>
where
    T: ?Sized,
{
    unsafe fn shallow_copy(&mut self) -> Self {
        Arc::from_raw(&**self as *const _)
    }
}

use std::rc::Rc;
impl<T> ShallowCopy for Rc<T>
where
    T: ?Sized,
{
    unsafe fn shallow_copy(&mut self) -> Self {
        Rc::from_raw(&**self as *const _)
    }
}

impl<T> ShallowCopy for Box<T>
where
    T: ?Sized,
{
    unsafe fn shallow_copy(&mut self) -> Self {
        Box::from_raw(&mut **self as *mut _)
    }
}

impl ShallowCopy for String {
    unsafe fn shallow_copy(&mut self) -> Self {
        let buf = self.as_bytes_mut().as_mut_ptr();
        let len = self.len();
        let cap = self.capacity();
        String::from_raw_parts(buf, len, cap)
    }
}

impl<T> ShallowCopy for Vec<T> {
    unsafe fn shallow_copy(&mut self) -> Self {
        let ptr = self.as_mut_ptr();
        let len = self.len();
        let cap = self.capacity();
        Vec::from_raw_parts(ptr, len, cap)
    }
}

impl<'a, T> ShallowCopy for &'a T
where
    T: ?Sized,
{
    unsafe fn shallow_copy(&mut self) -> Self {
        &*self
    }
}

macro_rules! impl_shallow_copy_for_copy {
    ($($t:ty)*) => ($(
        impl ShallowCopy for $t {
            unsafe fn shallow_copy(&mut self) -> Self {
                *self
            }
        }
    )*)
}

impl_shallow_copy_for_copy!(() bool char usize u8 u16 u32 u64 isize i8 i16 i32 i64);

macro_rules! tuple_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<$($T:ShallowCopy),+> ShallowCopy for ($($T,)+) {
                unsafe fn shallow_copy(&mut self) -> Self {
                    ($(self.$idx.shallow_copy(),)+)
                }
            }
        )+
    }
}

tuple_impls! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
    Tuple7 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
    }
    Tuple8 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }
    Tuple9 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }
    Tuple10 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }
    Tuple11 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }
    Tuple12 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }
}