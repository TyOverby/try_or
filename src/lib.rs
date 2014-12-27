#![feature(macro_rules)]

#[macro_export]
pub macro_rules! try_or(
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
        try_or!(always_bad(), Numb);
        try_or!(always_bad(), Other);
        Ok(())
    }

    #[test]
    fn test_something() {
        match translates() {
            Ok(_) => fail!("no!"),
            Err(Numb(_)) => {}
            Err(Other(_)) => {fail!("other no!")}
        }
    }

    fn fails_with_int() -> Result<(), uint> {
        Err(5)
    }

    fn modify_int() -> Result<(), uint> {
        try_or!(fails_with_int(), |x: uint| x + 1);
        Ok(())
    }

    #[test]
    fn test_modint() {
        match modify_int() {
            Ok(_) => fail!("not ok man.  not ok"),
            Err(6) => {},
            Err(x) => fail!("bad number, {}", x)
        }
    }
}
