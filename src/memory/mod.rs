use super::Error;
use std::collections::HashMap;

pub enum Cll {
    Int(Option<i32>),
    Flt(Option<f64>),
    Chr(Option<char>),
}

pub type Mem = HashMap<String, Cll>;
