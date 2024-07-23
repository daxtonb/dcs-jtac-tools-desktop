use std::{collections::HashMap, error::Error, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::display_settings::{self, DisplaySettings};

/// Represents the position of a multi-functional color display.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MfcdPosition {
    Left,
    Right,
    Center,
}

/// Represents a multi-functional color display.
pub struct MultiFunctionalColorDisplay {
    /// The x-position of the display.
    pub x_position: u16,
    /// The y-position of the display.
    pub y_position: u16,
    /// The width of the display.
    pub width: u16,
    /// The height of the display.
    pub height: u16,
}

impl MultiFunctionalColorDisplay {
    /// Reads the contents of a file and returns a `HashMap` of `MultiFunctionalColorDisplay` objects.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the file to be read.
    ///
    /// # Returns
    ///
    /// A Result containing a HashMap of `MultiFunctionalColorDisplay` objects, with the `MfcdPosition` as the key and the `MultiFunctionalColorDisplay` as the value.
    /// If the file cannot be read or an error occurs, an Err containing a Box<dyn Error> is returned.
    pub fn all_from_file(
        file_path: &PathBuf,
        display_settings: &DisplaySettings,
    ) -> Result<HashMap<MfcdPosition, MultiFunctionalColorDisplay>, Box<dyn Error>> {
        match fs::read_to_string(file_path) {
            Ok(contents) => {
                let mut rv = HashMap::new();
                let positions = vec![
                    MfcdPosition::Left,
                    MfcdPosition::Right,
                    MfcdPosition::Center,
                ];
                for position in positions {
                    let mfcd = get_from_file_contents(&position, &contents, &display_settings);
                    if let Some(mfcd) = mfcd {
                        rv.insert(position, mfcd);
                    }
                }
                Ok(rv)
            }
            Err(err) => {
                eprintln!("Error reading file at {:?}: {:?}", &file_path, err);
                Err(err.to_string().into())
            }
        }
    }
}

/// Retrieves the multi-functional color display settings from the file contents.
///
/// # Arguments
///
/// * `position` - The position of the multi-functional color display.
/// * `contents` - The contents of the file.
///
/// # Returns
///
/// Returns an `Option` containing the `MultiFunctionalColorDisplay` instance if the settings are found, or `None` if the settings are not found.
fn get_from_file_contents(
    position: &MfcdPosition,
    contents: &str,
    display_settings: &DisplaySettings,
) -> Option<MultiFunctionalColorDisplay> {
    let prefix = match position {
        MfcdPosition::Left => "LEFT",
        MfcdPosition::Right => "RIGHT",
        MfcdPosition::Center => "CENTER",
    };
    // TODO: handle dynamic case "scree.width / 2"
    let regex = regex::Regex::new(&format!(
        r"{}_MFCD =\s*\{{\s*x = (\d+|screen\.width|screen\.width \/ \d+);\s*y = (\d+|screen\.height|screen\.height \/ \d+);\s*width = (\d+|screen\.width|screen\.width \/ \d+);\s*height = (\d+|screen\.height|screen\.height \/ \d+);",
        prefix
    ))
    .unwrap();
    if let Some(captures) = regex.captures(contents) {
        Some(MultiFunctionalColorDisplay {
            x_position: extract_and_parse_integer_from_group(&captures, 1).unwrap(),
            y_position: extract_and_parse_integer_from_group(&captures, 2).unwrap(),
            width: extract_and_parse_integer_from_group(&captures, 3).unwrap(),
            height: extract_and_parse_integer_from_group(&captures, 4).unwrap(),
        })
    } else {
        None
    }
}

/// Extracts and parses a group from regex captures as an integer.
///
/// # Arguments
///
/// * `captures` - The regex captures.
/// * `group_index` - The index of the group to extract and parse.
///
/// # Returns
///
/// Returns a `Result` containing the parsed value if successful, or an error if parsing fails.
fn extract_and_parse_integer_from_group(
    captures: &regex::Captures,
    group_index: usize,
) -> Result<u16, Box<dyn Error>> {
    match captures.get(group_index) {
        Some(capture) => match capture.as_str().parse::<u16>() {
            Ok(x) => Ok(x),
            Err(err) => {
                eprintln!(
                    "Failed to parse to integer for group {}: {:?}",
                    group_index, err
                );
                Err(err.to_string().into())
            }
        },
        None => Err(format!("Failed to find group {}", group_index).into()),
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn test_from_file_success() {
        let file_path = PathBuf::from("test_settings1.txt");
        let display_settings = DisplaySettings {
            width: 1920,
            height: 1080,
            profile_name: "test_profile".to_string(),
        };
        let contents = "LEFT_MFCD = 
        {
            x = 10;
            y = 20;
            width = 100;
            height = 200;
        }

        RIGHT_MFCD = 
        {
            x = 30;
            y = 40;
            width = 150;
            height = 250;
        }

        CENTER_MFCD = 
        {
            x = 50;
            y = 60;
            width = 200;
            height = 300;
        }";
        fs::write(&file_path, contents).unwrap();

        let settings =
            MultiFunctionalColorDisplay::all_from_file(&file_path, &display_settings).unwrap();

        assert!(settings.contains_key(&MfcdPosition::Left));
        let left_mfcd = settings.get(&MfcdPosition::Left).unwrap();
        assert_eq!(left_mfcd.x_position, 10);
        assert_eq!(left_mfcd.y_position, 20);
        assert_eq!(left_mfcd.width, 100);
        assert_eq!(left_mfcd.height, 200);

        assert!(settings.contains_key(&MfcdPosition::Right));
        let right_mfcd = settings.get(&MfcdPosition::Right).unwrap();
        assert_eq!(right_mfcd.x_position, 30);
        assert_eq!(right_mfcd.y_position, 40);
        assert_eq!(right_mfcd.width, 150);
        assert_eq!(right_mfcd.height, 250);

        assert!(settings.contains_key(&MfcdPosition::Center));
        let center_mfcd = settings.get(&MfcdPosition::Center).unwrap();
        assert_eq!(center_mfcd.x_position, 50);
        assert_eq!(center_mfcd.y_position, 60);
        assert_eq!(center_mfcd.width, 200);
        assert_eq!(center_mfcd.height, 300);

        fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_from_file_with_partial_mfds_defined_success() {
        let file_path = PathBuf::from("test_settings2.txt");
        let display_settings = DisplaySettings {
            width: 1920,
            height: 1080,
            profile_name: "test_profile".to_string(),
        };
        let contents = "
        RIGHT_MFCD = 
        {
            x = 30;
            y = 40;
            width = 150;
            height = 250;
        }
        ";
        fs::write(&file_path, contents).unwrap();

        let settings =
            MultiFunctionalColorDisplay::all_from_file(&file_path, &display_settings).unwrap();

        assert!(settings.contains_key(&MfcdPosition::Right));
        let right_mfcd = settings.get(&MfcdPosition::Right).unwrap();
        assert_eq!(right_mfcd.x_position, 30);
        assert_eq!(right_mfcd.y_position, 40);
        assert_eq!(right_mfcd.width, 150);
        assert_eq!(right_mfcd.height, 250);

        assert!(!settings.contains_key(&MfcdPosition::Left));
        assert!(!settings.contains_key(&MfcdPosition::Center));

        fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_from_file_error() {
        let file_path = PathBuf::from("nonexistent_file.txt");
        let display_settings = DisplaySettings {
            width: 1920,
            height: 1080,
            profile_name: "test_profile".to_string(),
        };

        let result = MultiFunctionalColorDisplay::all_from_file(&file_path, &display_settings);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_from_file_contents_found() {
        let display_settings = DisplaySettings {
            width: 1920,
            height: 1080,
            profile_name: "test_profile".to_string(),
        };
        let contents = "LEFT_MFCD = 
        {
            x = 10;
            y = 20;
            width = 100;
            height = 200;
        }";
        let position = MfcdPosition::Left;

        let result = get_from_file_contents(&position, contents, &display_settings);

        assert!(result.is_some());
        let mfcd = result.unwrap();
        assert_eq!(mfcd.x_position, 10);
        assert_eq!(mfcd.y_position, 20);
        assert_eq!(mfcd.width, 100);
        assert_eq!(mfcd.height, 200);
    }

    #[test]
    fn test_get_from_file_contents_not_found() {
        let display_settings = DisplaySettings {
            width: 1920,
            height: 1080,
            profile_name: "test_profile".to_string(),
        };
        let contents = "RIGHT_MFCD = 
        {
            x = 30;
            y = 40;
            width = 150;
            height = 250;
        }";
        let position = MfcdPosition::Left;

        let result = get_from_file_contents(&position, contents, &display_settings);

        assert!(result.is_none());
    }

    #[test]
    fn test_extract_and_parse_group_success() {
        let regex = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
        let captures = regex.captures("10 20 100 200").unwrap();
        let group_index = 1;

        let result = extract_and_parse_integer_from_group(&captures, group_index);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_extract_and_parse_group_error() {
        let regex = Regex::new(r"(\w+) (\d+) (\d+) (\d+)").unwrap();
        let captures = regex.captures("invalid 20 100 200").unwrap();
        let group_index = 1;

        let result = extract_and_parse_integer_from_group(&captures, group_index);

        assert!(result.is_err());
    }
}
