pub trait Unsigned {}
pub trait Mode {}
pub trait Formula {
    type Output: Formula;
}
