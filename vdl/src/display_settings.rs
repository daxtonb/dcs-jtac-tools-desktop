use std::{error::Error, fs, path::PathBuf};

use regex::Regex;

/// Represents the display settings, including width, height, and profile name.
#[derive(Debug)]
/// Represents the display settings for the application.
pub struct DisplaySettings {
    /// The width of the display.
    pub width: u16,
    /// The height of the display.
    pub height: u16,
    /// The name of the profile associated with the display settings.
    pub profile_name: String,
}

impl DisplaySettings {
    /// Creates a new `DisplaySettings` instance by reading the settings from a file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the file containing the display settings.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `DisplaySettings` instance if successful, or an error if reading the file or parsing the settings fails.
    pub fn from_file(file_path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        match fs::read_to_string(file_path) {
            Ok(contents) => Ok(DisplaySettings {
                height: get_height_from_file_contents(&contents)?,
                width: get_width_from_file_contents(&contents)?,
                profile_name: get_display_profile_from_file_contents(&contents)?,
            }),
            Err(err) => {
                eprintln!("Error reading file at {:?}: {:?}", &file_path, err);
                Err(err.to_string().into())
            }
        }
    }
}

/// Retrieves the height setting from the file contents.
///
/// # Arguments
///
/// * `contents` - The contents of the file.
///
/// # Returns
///
/// Returns the height value if found and successfully parsed, or an error if the height setting is not found or cannot be parsed.
fn get_height_from_file_contents(contents: &String) -> Result<u16, Box<dyn Error>> {
    let height_regex: Regex = Regex::new(r#"\["height"\] = (\d+)"#).unwrap();
    let height = match height_regex.captures(contents) {
        Some(captures) => match captures.get(1) {
            Some(capture) => match capture.as_str().parse::<u16>() {
                Ok(height) => height,
                Err(err) => {
                    return Err(format!("Failed to parse height to integer: {:?}", err).into())
                }
            },
            None => return Err("Failed to find graphics 'height' setting".into()),
        },
        None => return Err("Failed to find graphics 'height' setting".into()),
    };

    Ok(height)
}

/// Retrieves the width setting from the file contents.
///
/// # Arguments
///
/// * `contents` - The contents of the file.
///
/// # Returns
///
/// Returns the width value if found and successfully parsed, or an error if the width setting is not found or cannot be parsed.
fn get_width_from_file_contents(contents: &String) -> Result<u16, Box<dyn Error>> {
    let width_regex: Regex = Regex::new(r#"\["width"\] = (\d+)"#).unwrap();
    let width = match width_regex.captures(&contents) {
        Some(captures) => match captures.get(1) {
            Some(capture) => match capture.as_str().parse::<u16>() {
                Ok(height) => height,
                Err(err) => {
                    return Err(format!("Failed to parse width to integer: {:?}", err).into())
                }
            },
            None => return Err("Failed to find graphics 'width' setting".into()),
        },
        None => return Err("Failed to find graphics 'width' setting".into()),
    };

    Ok(width)
}

/// Retrieves the display profile setting from the file contents.
///
/// # Arguments
///
/// * `contents` - The contents of the file.
///
/// # Returns
///
/// Returns the display profile value if found, or an error if the display profile setting is not found.
fn get_display_profile_from_file_contents(contents: &String) -> Result<String, Box<dyn Error>> {
    let profile_regex: Regex =
        Regex::new(r#"\["multiMonitorSetup"\] = "([a-zA-Z0-9\+\(\)]+)""#).unwrap();
    let profile = match profile_regex.captures(&contents) {
        Some(captures) => match captures.get(1) {
            Some(capture) => capture.as_str(),
            None => return Err("Failed to find graphics 'multiMonitorSetup' setting".into()),
        },
        None => return Err("Failed to find graphics 'multiMonitorSetup' setting".into()),
    };

    Ok(profile.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_file_valid_contents() {
        let file_path = PathBuf::from("test_settings1.txt");
        let contents = r#"
            ["height"] = 1080
            ["width"] = 1920
            ["multiMonitorSetup"] = "extended"
        "#;
        fs::write(&file_path, contents).unwrap();

        let display_settings = DisplaySettings::from_file(&file_path).unwrap();

        assert_eq!(display_settings.height, 1080);
        assert_eq!(display_settings.width, 1920);
        assert_eq!(display_settings.profile_name, "extended");

        fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_from_file_missing_height() {
        let file_path = PathBuf::from("test_settings2.txt");
        let contents = r#"
            ["width"] = 1920
            ["multiMonitorSetup"] = "extended"
        "#;
        fs::write(&file_path, contents).unwrap();

        let result = DisplaySettings::from_file(&file_path);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to find graphics 'height' setting"
        );

        fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_from_file_invalid_height() {
        let file_path = PathBuf::from("test_settings3.txt");
        let contents = r#"
            ["height"] = "invalid"
            ["width"] = 1920
            ["multiMonitorSetup"] = "extended"
        "#;
        fs::write(&file_path, contents).unwrap();

        let result = DisplaySettings::from_file(&file_path);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to find graphics 'height' setting"
        );

        fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_from_file_missing_width() {
        let file_path = PathBuf::from("test_settings4.txt");
        let contents = r#"
            ["height"] = 1080
            ["multiMonitorSetup"] = "extended"
        "#;
        fs::write(&file_path, contents).unwrap();

        let result = DisplaySettings::from_file(&file_path);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to find graphics 'width' setting"
        );

        fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_from_file_invalid_width() {
        let file_path = PathBuf::from("test_settings5.txt");
        let contents = r#"
            ["height"] = 1080
            ["width"] = "invalid"
            ["multiMonitorSetup"] = "extended"
        "#;
        fs::write(&file_path, contents).unwrap();

        let result = DisplaySettings::from_file(&file_path);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to find graphics 'width' setting"
        );

        fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_from_file_missing_profile() {
        let file_path = PathBuf::from("test_settings6.txt");
        let contents = r#"
            ["height"] = 1080
            ["width"] = 1920
        "#;
        fs::write(&file_path, contents).unwrap();

        let result = DisplaySettings::from_file(&file_path);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to find graphics 'multiMonitorSetup' setting"
        );

        fs::remove_file(&file_path).unwrap();
    }
}
