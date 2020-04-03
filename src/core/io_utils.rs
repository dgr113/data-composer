use serde_yaml;
use serde_json;
use std::io;
use std::path::Path;



/// parse YAML file
pub fn parse_yaml(content: String) -> Result<serde_yaml::Value, io::Error> {
    serde_yaml::from_str(&content).or_else(|_| Err(io::Error::from(io::ErrorKind::BrokenPipe)))
}


/// dump YAML file
pub fn dump_yaml(content: serde_yaml::Value) -> Result<String, io::Error> {
    serde_yaml::to_string(&content).or_else(|_| Err(io::Error::from(io::ErrorKind::BrokenPipe)))
}


/// parse YAML file
pub fn parse_json(content: String) -> Result<serde_json::Value, String> {
    serde_json::from_str(&content).or_else(|err| Err(err.to_string()))
}


/// dump json file
pub fn dump_json(content: serde_json::Value) -> Result<String, io::Error> {
    serde_json::to_string(&content).or_else(|_| Err(io::Error::from(io::ErrorKind::BrokenPipe)))
}


/// Build filename from component
///
/// # Examples
/// ```
/// use content_machiner::core::io_utils::build_filename;
/// use data_composer::core::io_utils::build_filename;
/// let main_component = "movies";
/// let add_components = &["test", "ru-RU"];
/// let result = build_filename(main_component, Some(add_components), "yml", ".");
/// assert_eq!(result, "movies.test.ru-RU.yml");
/// ```
pub fn build_filename(main_component: &str, add_components: Option<&[&str]>, file_ext: &str, sep: &str) -> String {
    let mut comp_vector = vec![main_component, ];
    match add_components {
        Some(t) => comp_vector.extend_from_slice(t),
        None => ()
    }
    format!("{}.{}", comp_vector.join(sep), file_ext)
}


/// Build dyn path from component
pub fn build_path(basedir: &str, filename: &str) -> String {
    Path::new(basedir).join(filename).to_str().unwrap().to_string()
}
