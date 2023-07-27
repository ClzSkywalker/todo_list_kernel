pub trait IValueObject {
    type Output;
    fn same_value_as(&self, other: &Self::Output) -> bool;
}
