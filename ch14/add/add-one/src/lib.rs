
pub fn add_one( v:i32 ) -> i32{
    v + 1
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works(){
        assert_eq!(3, add_one(2));
    }
}
