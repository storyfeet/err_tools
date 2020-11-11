use thiserror::*;

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{} ({})",.e,.s)]
pub struct SWrap<E: std::error::Error> {
    e: E,
    s: &'static str,
}

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{} ({})",.e,.s)]
pub struct SgWrap<E: std::error::Error> {
    e: E,
    s: String,
}

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{}",.0)]
pub struct SError(&'static str);
#[derive(Error, Debug, Clone, PartialEq)]
#[error("{}",.0)]
pub struct SgError(String);

pub trait OpError {
    type V;
    fn op_err(self) -> Option<Self::V>;
}

impl<V> OpError for Option<V> {
    type V = V;
    fn op_err(self) -> Self {
        self
    }
}
