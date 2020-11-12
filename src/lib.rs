use std::fmt::{Debug, Display};
use thiserror::*;

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{} ({})",.e,.s)]
pub struct SWrap<E: Debug + Display> {
    e: E,
    s: &'static str,
}

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{} ({})",.e,.s)]
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

pub fn e_str<T>(s: &'static str) -> anyhow::Result<T> {
    Err(SError(s).into())
}

pub fn e_string<T>(s: String) -> anyhow::Result<T> {
    Err(SgError(s).into())
}
pub trait OpError: Sized {
    type V;
    fn op_err(self) -> Option<Self::V>;
    fn e_str(self, s: &'static str) -> anyhow::Result<Self::V> {
        self.op_err().ok_or(SError(s).into())
    }
    fn e_string(self, s: String) -> anyhow::Result<Self::V> {
        self.op_err().ok_or(SgError(s).into())
    }
}

impl<V> OpError for Option<V> {
    type V = V;
    fn op_err(self) -> Self {
        self
    }
}

pub trait ResError: Sized {
    type V;
    type E: Debug + Display + Send + Sync + 'static;
    fn res_err(self) -> Result<Self::V, Self::E>;
    fn e_str(self, s: &'static str) -> anyhow::Result<Self::V> {
        self.res_err().map_err(|e| SWrap { s, e }.into())
    }
    fn e_string(self, s: String) -> anyhow::Result<Self::V> {
        self.res_err().map_err(|e| SgWrap { s, e }.into())
    }
}

impl<T, E: Debug + Display + Sync + Send + 'static> ResError for Result<T, E> {
    type V = T;
    type E = E;
    fn res_err(self) -> Self {
        self
    }
}
