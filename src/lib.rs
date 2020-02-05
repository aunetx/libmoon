pub mod instructions;
mod io;
pub mod runtime;
pub use io::*;
pub use memory::*;
pub use runtime::*;
pub mod memory;

/// Identifier used to check if `val` is a `var` or a `value` :\
/// `&` is the default one
const VARIABLE_IDENTIFIER: &str = "&";
