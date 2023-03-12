pub trait Number: PartialOrd + num_traits::Signed + Copy {}
impl<T: PartialOrd + num_traits::Signed + Copy> Number for T {}

// Lower/Upper Limits Structure
pub struct Limits<T: Number> {
    pub lower_limit: T,
    pub upper_limit: T,
}
