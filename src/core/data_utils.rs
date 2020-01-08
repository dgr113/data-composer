use serde_yaml;



/// Build vector from first element and sliced others
pub fn extend_slice<T: Clone>(first: T, additions: &[T]) -> Vec<T> {
    let mut v = vec![first, ];
    v.extend(additions.iter().cloned()); v
}


/// Build Yaml compose key from components
pub fn build_compose_key(keys: &[&str]) -> serde_yaml::Value {
    serde_yaml::to_value(keys).unwrap()
}


/// Unwrap Optional value to &str
pub fn unwrap_to_str(obj: Option<String>, default: &str) -> String {
    obj.unwrap_or(default.to_string())
}
