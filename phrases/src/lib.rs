#[cfg(test)]
mod test {
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn it_works() {
        assert!(false);
    }
}

pub mod chinese;
pub mod english;
pub mod japanese;
