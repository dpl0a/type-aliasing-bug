pub trait Cache {
    fn new() -> Self;
}

pub struct BarStruct<C: Cache> {
    t: C,
}

pub struct FooStruct {}
impl Cache for FooStruct {
    fn new() -> Self {
        FooStruct {}
    }
}

trait Whatever {
    fn some_function(&self) -> Result<usize, FooError>;
}

impl<C: Cache> BarStruct<C> {
    pub fn new() -> Self {
        BarStruct { t: C::new() }
    }
    pub fn const1(&self) -> usize {
        1
    }
}

impl<C: Cache> Whatever for BarStruct<C> {
    fn some_function(&self) -> Result<usize, FooError> {
        Err(FooError::One)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum FooError {
    One,
}

#[cfg(test)]
mod tests {
    use crate::{FooError, Whatever};
    use foo::TestBar;

    #[test]
    fn footest() {
        struct A;

        impl A {
            pub fn bar<T: Whatever>(t: T) -> Result<usize, FooError> {
                t.some_function()
            }
        }

        let foo = TestBar::new();
        assert_eq!(A::bar(foo), Err(FooError::One))
    }
}
