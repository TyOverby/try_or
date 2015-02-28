
#[macro_export]
macro_rules! try_or(
    ($e:expr) => (
        match $e {
            Ok(e) => e,
            Err(e) => return Err(e)

        }
    );
    ($e:expr, $c:expr) => (
        match $e {
            Ok(e) => e,
            Err(e) => {
                return Err(($c)(e))
            }
        }
    );
    ($e:expr, $c:ident) => (
        match $e {
            Ok(e) => e,
            Err(e) => {
                return Err($c(e))
            }
        }
    );
);

#[cfg(test)]
mod test {
    enum SillyError {
        Numb(BadThing),
        Other(BadThing)
    }
    struct BadThing;

    fn always_bad() -> Result<(), BadThing> {
        Err(BadThing)
    }

    fn translates() -> Result<(), SillyError> {
        try_or!(always_bad(), SillyError::Numb);
        try_or!(always_bad(), SillyError::Other);
        Ok(())
    }

    #[test]
    fn test_something() {
        match translates() {
            Ok(_) => panic!("no!"),
            Err(SillyError::Numb(_)) => {}
            Err(SillyError::Other(_)) => {panic!("other no!")}
        }
    }

    fn fails_with_int() -> Result<(), u32> {
        Err(5)
    }

    fn modify_int() -> Result<(), u32> {
        try_or!(fails_with_int(), |x: u32| x + 1);
        Ok(())
    }

    #[test]
    fn test_modint() {
        match modify_int() {
            Ok(_) => panic!("not ok man.  not ok"),
            Err(6) => {},
            Err(x) => panic!("bad number, {}", x)
        }
    }
}
