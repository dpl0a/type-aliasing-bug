pub trait Cache {
    fn new() -> Self;
}

pub struct Program<C: Cache> {
    t: C
}

pub struct CBNCache {}
impl Cache for CBNCache {
    fn new() -> Self {
        CBNCache {}
    }
}

trait Whatever {
    fn some_function(&self) -> Result<usize, FooError>;
}

impl<C: Cache> Program<C> {
    pub fn new() -> Self {
        Program { t: C::new() }
    }
    pub fn const1(&self) -> usize {
        1
    }
}

impl<C: Cache> Whatever for Program<C> {
    fn some_function(&self) -> Result<usize, FooError> {
            Err(FooError::One)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum FooError {
    One,
    Two { b: bool }
}

#[cfg(test)]
mod tests {
    use foo::TestProgram;
    use crate::{FooError, Whatever};

    #[test]
    fn footest() {
        struct A;

        impl A {
            pub fn bar<T: Whatever>(t: T) -> Result<usize, FooError> {
                t.some_function()
            }
        }

        let foo = TestProgram::new();
        assert_eq!(A::bar(foo), Err(FooError::One))
    }
}