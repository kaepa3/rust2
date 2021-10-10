





pub mod kinds{
    pub enum PrimaryColor{
        Red,
        Yellow,
        Blue,
    }
    pub enum SecondaryColor{
        Orange,
        Green,
        Purple,
    }
}

pub mod utils{
    use crate::kinds::*;
    pub fn mix (c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor{
        SecondaryColor::Orange
    }
}
/// Add one to the number give.
///
/// # Example
/// let x = 5;
/// assert_eq!(6, add_one(5));
///
///
pub fn add_one(v: i32) -> i32 {
    v + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
