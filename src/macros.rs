/**
* Create an anyhow::Err using format args
*
* ```
*   #[macro_use]
*   use err_tools::*;
*
*   fn do_thing(n:i32)->anyhow::Result<i32>{
*       return e_format!("a b {}",n);
*   }
*
*   assert_eq!(
*       "a b 3",
*       do_thing(3).err().unwrap().to_string()
*   );
* ```
*/
#[macro_export]
macro_rules! e_format {
    ($($arg:tt)*) => {
        Err(SgError(format!($($arg)*)).into())
    };
}

/**
* Create a traceable result, ready to return as error from function
*
* The return type is always : Result<_,AtError>)
* So to Ok() Type will need to be indicated somewhere else.
*
*/
#[macro_export]
macro_rules! e_trace {
    ($lit:literal) => {
        Err(AtError {
            loc: Location{ file:file!(), line:line!()},
            e_type: ErrType::S($lit),
        })
    };
    ($err:expr) => {
        Err(AtError{
            loc: Location{ file:file!(), line:line!()},
            e_type: ErrType::Any($err.into()),
        })
    };
    ($($arg:tt)*) => {
        Err(AtError {
            loc: Location{ file:file!(), line:line!()},
            e_type: ErrType::ST(format!($($arg)*)),
        })
    };
}

#[cfg(test)]
mod tests {
    use crate::stackable::*;
    use std::io;
    #[test]
    fn test_can_create_str_errs_with_e_trace() {
        let r: Result<i32, AtError> = e_trace!("hello");
        let next_line = line!();

        let e = r.err().unwrap();

        //Assert the location is correct
        assert_eq!(next_line - 1, e.loc.line);
        match e.e_type {
            ErrType::S(s) => assert_eq!("hello", s),
            ErrType::ST(_) => panic!("Expected 'str' not 'String'"),
            ErrType::Any(_) => panic!("Expected 'str' not 'ANY'"),
        }
    }

    #[test]
    fn test_can_create_string_errs_with_e_trace() {
        let r: Result<i32, AtError> = e_trace!("hello {}", 24);
        let next_line = line!();

        let e = r.err().unwrap();

        //Assert the location is correct
        assert_eq!(next_line - 1, e.loc.line);
        match e.e_type {
            ErrType::S(_) => panic!("Expected 'String' not 'Str'"),
            ErrType::ST(s) => assert_eq!("hello 24", s),
            ErrType::Any(_) => panic!("Expected 'str' not 'ANY'"),
        }
    }

    #[test]
    fn test_can_create_any_errs_with_e_trace() {
        let r: Result<i32, AtError> =
            e_trace!(io::Error::new(io::ErrorKind::FileTooLarge, "file too big"));
        let next_line = line!();

        let e = r.err().unwrap();

        //Assert the location is correct
        assert_eq!(next_line - 1, e.loc.line);
        match e.e_type {
            ErrType::S(_) => panic!("Expected 'String' not 'Str'"),
            ErrType::ST(_) => panic!("Expected 'str' not 'String'"),
            ErrType::Any(any) => assert_eq!("file too big", any.to_string()),
        }
    }
}
