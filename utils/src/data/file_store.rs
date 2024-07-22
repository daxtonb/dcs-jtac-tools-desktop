use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};

/// A trait representing a file store that can read and write data to a file.
pub trait FileStore<T: Serialize + DeserializeOwned> {
    /// Retrieves the value stored in the file at the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file.
    ///
    /// # Returns
    ///
    /// An `Option` containing the deserialized value if the file exists and can be read, or `None` otherwise.
    fn get(path: &str) -> Option<T> {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let value = serde_json::from_reader(reader).ok()?;
                Some(value)
            }
            Err(err) => {
                eprintln!("Error opening file: {:?}", err);
                None
            }
        }
    }

    /// Stores the given value in a file at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file.
    /// * `value` - The value to be serialized and stored in the file.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the operation was successful or an error occurred.
    fn set(path: &str, value: &T) -> Result<(), Box<dyn Error>> {
        match File::create(path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                match serde_json::to_writer(writer, value) {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        eprintln!("Error writing to file: {:?}", err);
                        Err(err.into())
                    }
                }
            }
            Err(err) => {
                eprintln!("Error creating file: {:?}", err);
                Err(err.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    struct TestFileStore {}
    impl FileStore<u32> for TestFileStore {}

    #[test]
    fn test_get_existing_file() {
        let path = "data.json";
        let value = 42;
        fs::write(path, value.to_string()).unwrap();
        let stored_value = fs::read_to_string(path)
            .ok()
            .and_then(|s| s.parse::<u32>().ok());
        assert_eq!(stored_value.unwrap(), 42);
    }

    #[test]
    fn test_get_nonexistent_file() {
        let path = "nonexistent.json";
        let value = TestFileStore::get(path);
        assert_eq!(value, None);
    }

    #[test]
    fn test_set_file_creation() {
        let path = "new_file.json";
        let value = 123;
        let result = TestFileStore::set(path, &value);
        assert!(result.is_ok());
        assert!(std::path::Path::new(path).exists());
    }

    #[test]
    fn test_set_file_content() {
        let path = "new_file.json";
        let value = 123;
        TestFileStore::set(path, &value).unwrap();
        let stored_value = TestFileStore::get(path);
        assert_eq!(stored_value, Some(value));
    }
}
