pub use crate::macros;
use thiserror::*;

pub type TraceResult<T> = Result<T, TraceError>;

/**
* An Error type that contains trace information
*/
#[derive(Error, Debug)]
pub struct TraceError(pub Vec<AtError>);

#[derive(Error, Debug)]
#[error("{}:{}",.file,.line)]
pub struct Location {
    pub file: &'static str,
    pub line: u32,
}

#[derive(Error, Debug)]
#[error("{} - {}",.loc,.e_type)]
pub struct AtError {
    pub loc: Location,
    pub e_type: ErrType,
}

#[derive(Error, Debug)]
pub enum ErrType {
    S(&'static str),
    ST(String),
    Any(anyhow::Error),
}

impl std::fmt::Display for TraceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for l in &self.0 {
            writeln!(f, "{}", l)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        return match self {
            ErrType::S(s) => write!(f, "{}", s),
            ErrType::ST(s) => write!(f, "{}", s),
            ErrType::Any(s) => write!(f, "{}", s),
        };
    }
}

pub trait Traceable: Sized {
    fn push_err(self, e: AtError) -> TraceError;
    fn push_str(self, s: &'static str, l: Location) -> TraceError {
        self.push_err(AtError {
            e_type: ErrType::S(s),
            loc: l,
        })
    }
    fn push_string(self, s: String, l: Location) -> TraceError {
        self.push_err(AtError {
            e_type: ErrType::ST(s),
            loc: l,
        })
    }
    fn push_any(self, a: anyhow::Error, l: Location) -> TraceError {
        self.push_err(AtError {
            e_type: ErrType::Any(a),
            loc: l,
        })
    }
}

pub trait TraceableRes<T>: Sized {
    fn push_err(self, e: AtError) -> TraceResult<T>;
    fn push_str(self, s: &'static str, l: Location) -> TraceResult<T> {
        self.push_err(AtError {
            e_type: ErrType::S(s),
            loc: l,
        })
    }
    fn push_string(self, s: String, l: Location) -> TraceResult<T> {
        self.push_err(AtError {
            e_type: ErrType::ST(s),
            loc: l,
        })
    }
    fn push_any(self, a: anyhow::Error, l: Location) -> TraceResult<T> {
        self.push_err(AtError {
            e_type: ErrType::Any(a),
            loc: l,
        })
    }
}

impl<T, E: Traceable> TraceableRes<T> for Result<T, E> {
    fn push_err(self, err: AtError) -> TraceResult<T> {
        self.map_err(|e| e.push_err(err))
    }
}

impl Traceable for TraceError {
    fn push_err(mut self, e_new: AtError) -> TraceError {
        self.0.push(e_new);
        self
    }
}

impl Traceable for AtError {
    fn push_err(self, e_new: AtError) -> TraceError {
        TraceError(vec![self, e_new])
    }
}

impl Traceable for anyhow::Error {
    fn push_err(self, err: AtError) -> TraceError {
        TraceError(vec![
            AtError {
                e_type: ErrType::Any(self),
                loc: Location {
                    file: "--",
                    line: 0,
                },
            },
            err,
        ])
    }
}

impl Into<TraceError> for AtError {
    fn into(self) -> TraceError {
        TraceError(vec![self])
    }
}
