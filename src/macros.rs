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
macro_rules! err_at {
    ($lit:literal) => {
        AtError {
            loc: Location{ file:file!(), line:line!()},
            e_type: ErrType::S($lit),
        }.into()
    };
    ($err:expr) => {
        AtError{
            loc: Location{ file:file!(), line:line!()},
            e_type: ErrType::Any($err.into()),
        }.into()
    };
    ($($arg:tt)*) => {
        AtError {
            loc: Location{ file:file!(), line:line!()},
            e_type: ErrType::ST(format!($($arg)*)),
        }.into()
    };
}

/**
* Applies a location to the given Error
*
* Intended to be used as part of Result::map_err
* eg : ```err.map_err(err_at_map!())```
*/
#[macro_export]
macro_rules! err_at_map {
    () => {
        |e| err_at!(e)
    };
}

#[macro_export]
macro_rules! err_at_res {
    ($($arg:tt)*) => {
        Err( err_at!($($arg)*))
    };
}

#[macro_export]
macro_rules! err_wrap {
    ($($arg:tt)*) => {
        |e| Into::<TraceError>::into(e).push_err(err_at!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use crate::{macros::tests, traceable::*};
    use std::io;
    #[test]
    fn test_can_create_str_errs_with_err_att() {
        let r: Result<i32, AtError> = err_at_res!("hello");
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
    fn test_can_create_string_errs_with_err_at() {
        let r: Result<i32, AtError> = err_at_res!("hello {}", 24);
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
    fn test_can_create_any_errs_with_err_at() {
        let r: Result<i32, AtError> =
            err_at_res!(io::Error::new(io::ErrorKind::FileTooLarge, "file too big"));
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

    #[test]
    fn test_can_map_err_to_location() {
        let res: Result<i32, AtError> =
            Err(io::Error::new(io::ErrorKind::FileTooLarge, "file too big")).map_err(err_at_map!());
        let err_line = line!() - 1;

        let e = res.err().unwrap();
        assert_eq!(err_line, e.loc.line);
        match e.e_type {
            ErrType::S(_) | ErrType::ST(_) => panic!("Expected 'Any Error', got string based"),
            ErrType::Any(any) => assert_eq!("file too big", any.to_string()),
        }
    }
}
