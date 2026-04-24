use serde::Serializer;
use std::fmt::Display;

pub fn vec_to_csv_format<T: Display, S: Serializer>(v: &[T], s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&join_as_csv(v))
}

pub fn join_as_csv<T: Display>(v: &[T]) -> String {
    v.iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
