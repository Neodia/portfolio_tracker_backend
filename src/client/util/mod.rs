mod traits;
pub use traits::*;

use std::fmt::Display;
pub fn join_as_csv<T: Display>(v: &[T]) -> String {
    v.iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
