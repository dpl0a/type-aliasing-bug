use type_aliasing_bug::{BarStruct, FooStruct};

pub type C = FooStruct;
pub type TestBar = BarStruct<C>;
