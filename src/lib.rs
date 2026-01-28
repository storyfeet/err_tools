pub mod macros;
pub mod stackable;
use std::fmt::{Debug, Display};
use thiserror::*;

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{} ({})",.s,.e)]
pub struct SWrap<E: Debug + Display> {
    e: E,
    s: &'static str,
}

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{} ({})",.s,.e)]
pub struct SgWrap<E: Debug + Display> {
    e: E,
    s: String,
}

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{}",.0)]
pub struct SError(pub &'static str);
#[derive(Error, Debug, Clone, PartialEq)]
#[error("{}",.0)]
pub struct SgError(pub String);

/**
* Create an anyhow result from a static str
* Userful for quick comparisons
*/
pub fn e_str<T>(s: &'static str) -> anyhow::Result<T> {
    Err(SError(s).into())
}

/**
* Create an anyhow result from a String
* Userful for putting info in the string
*/
pub fn e_string<T>(s: String) -> anyhow::Result<T> {
    Err(SgError(s).into())
}

/**
* This trait applies to any Option and allows easy handling error cases.
*/
pub trait OpError: Sized {
    type V;
    /**
     * Convert option to result with 'static str error
     *
     * ```
     *
     * use err_tools::*;
     * fn may_error(op:Option<i32>)-> anyhow::Result<i32> {
     *   let n = op.e_str("option must be provided")?;
     *   Ok(n + 2)
     * }
     *
     * assert_eq!(5, may_error(Some(3)).unwrap());
     *
     * assert_eq!("option must be provided", may_error(None).err().unwrap().to_string());
     *
     * ```
     */
    fn e_str(self, s: &'static str) -> anyhow::Result<Self::V>;

    /**
     * Convert option to result with error from String
     *
     * ```
     *
     * use err_tools::*;
     * fn may_error(op:Option<i32>,other:i32)-> anyhow::Result<i32> {
     *   let n = op.e_string(format!("option must be provided with {}", other))?;
     *   Ok(n + other)
     * }
     *
     * assert_eq!(11, may_error(Some(3),8).unwrap());
     *
     * assert_eq!("option must be provided with 10", may_error(None,10).err().unwrap().to_string());
     *
     * ```
     */
    fn e_string(self, s: String) -> anyhow::Result<Self::V>;
}

/**
* A trait to make it easy to wrap errors with string errors on the same line
*/
impl<V> OpError for Option<V> {
    type V = V;
    fn e_str(self, s: &'static str) -> anyhow::Result<Self::V> {
        self.ok_or(SError(s).into())
    }
    fn e_string(self, s: String) -> anyhow::Result<Self::V> {
        self.ok_or(SgError(s).into())
    }
}

pub trait ResError: Sized {
    type V;
    type E: Debug + Display + Send + Sync + 'static;

    /**
     * Convert option to result with error from String
     *
     * ```
     *
     * use err_tools::*;
     * fn may_error(res:Result<i32,SError>)-> anyhow::Result<i32> {
     *   let n = res.e_str("OK must be provided")?;
     *   Ok(n + 2)
     * }
     *
     * assert_eq!(5, may_error(Ok(3)).unwrap());
     *
     * assert_eq!("OK must be provided (no num)",
     *   may_error(Err(SError("no num"))).err().unwrap().to_string()
     *   );
     *
     * ```
     */
    fn e_str(self, s: &'static str) -> anyhow::Result<Self::V>;

    /**
     * Convert option to result with error from String
     *
     * ```
     *
     * use err_tools::*;
     * fn may_error(res:Result<i32,SError>,other:i32)-> anyhow::Result<i32> {
     *   let n = res.e_string(format!("OK must be provided with {}",other))?;
     *   Ok(n + other)
     *
     * }
     *
     * assert_eq!(9, may_error(Ok(3),6).unwrap());
     *
     * assert_eq!("OK must be provided with 10 (no num)",
     *   may_error(Err(SError("no num")),10).err().unwrap().to_string()
     *   );
     *
     * ```
     */
    fn e_string(self, s: String) -> anyhow::Result<Self::V>;
}

impl<T, E: Debug + Display + Sync + Send + 'static> ResError for Result<T, E> {
    type V = T;
    type E = E;
    fn e_str(self, s: &'static str) -> anyhow::Result<Self::V> {
        self.map_err(|e| SWrap { s, e }.into())
    }
    fn e_string(self, s: String) -> anyhow::Result<Self::V> {
        self.map_err(|e| SgWrap { s, e }.into())
    }
}
