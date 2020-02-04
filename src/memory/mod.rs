use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Cll {
    Int(Option<i32>),
    Flt(Option<f64>),
    Chr(Option<char>),
}

impl fmt::Display for Cll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cll::Int(None) => write!(f, "unitialized (int)"),
            Cll::Flt(None) => write!(f, "unitialized (flt)"),
            Cll::Chr(None) => write!(f, "unitialized (chr)"),
            Cll::Int(Some(val)) => write!(f, "{} (int)", val),
            Cll::Flt(Some(val)) => write!(f, "{} (flt)", val),
            Cll::Chr(Some(val)) => write!(f, "{} (chr)", val),
        }
    }
}

pub type Mem = HashMap<String, Cll>;
