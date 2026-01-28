use thiserror::*;

pub type TraceResult<T> = Result<T, TraceError>;

/**
* An Error type that contains trace information
*/
#[derive(Error, Debug)]
pub struct TraceError(Vec<AtError>);

#[derive(Error, Debug)]
#[error("{}:{}",.0,.1)]
pub struct Location(pub &'static str, pub u32);

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

impl TraceError {
    fn push_err(mut self, e: AtError) -> Self {
        self.0.push(e);
        return self;
    }
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

pub trait TraceableRes<T>: Sized {
    fn push_err(self, err: AtError) -> TraceResult<T>;

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

impl<T> TraceableRes<T> for TraceResult<T> {
    fn push_err(self, err: AtError) -> TraceResult<T> {
        self.map_err(|e| e.push_err(err))
    }
}

impl<T> TraceableRes<T> for Result<T, AtError> {
    fn push_err(self, e_new: AtError) -> TraceResult<T> {
        self.map_err(|e_orig| TraceError(vec![e_orig, e_new]))
    }
}

impl<T> TraceableRes<T> for anyhow::Result<T> {
    fn push_err(self, err: AtError) -> TraceResult<T> {
        self.map_err(|e| {
            TraceError(vec![
                AtError {
                    e_type: ErrType::Any(e),
                    loc: Location("--", 0),
                },
                err,
            ])
        })
    }
}
