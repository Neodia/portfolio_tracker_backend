use serde::Serializer;

pub fn vec_to_csv_format<S: Serializer>(v: &[String], s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&v.join(","))
}
