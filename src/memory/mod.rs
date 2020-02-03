use std::collections::HashMap;

#[derive(Clone)]
pub enum Cll {
    Int(Option<i32>),
    Flt(Option<f64>),
    Chr(Option<char>),
}

pub type Mem = HashMap<String, Cll>;
