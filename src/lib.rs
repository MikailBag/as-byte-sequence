pub unsafe trait AsByteSequence: Clone {}

macro_rules! impl_trait {
    ($($ty:ty)*) => {
        $(
            unsafe impl $crate::AsByteSequence for $ty {}
        )*
    };
}

// All numeric primitive types are good
impl_trait!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize);
impl_trait!(f32 f64);
// Some special types which are good too
// TODO: implement for ! when it is stable
impl_trait!(());
// We do not implement this trait automatically for pointers, because they are only valid in context
// of one process, and sending pointer via e.g. networking is probably error

#[cfg(test)]
mod tests {
    use super::*;
    fn check<T: AsByteSequence>(_val: T) {}

    #[test]
    fn check_trait_is_implemented() {
        check(3 as u8);
        check(3 as i32);
        check(3 as usize);
        check(0.0 as f32);
        check(());
    }
}