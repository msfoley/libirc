use std::fmt;
use std::error::Error;
use std::path::Path;

pub struct ConfigReadError {
    description: String
}

impl ConfigReadError {
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(file: P, error_string: S) -> Self {
        let file_str = match file.as_ref().to_str() {
            Some(f) => f,
            None => "UKNOWN"
        };

        ConfigReadError {
            description: format!("Error parsing config {}: {}", file_str, error_string.as_ref())
        }
    }
}

impl Error for ConfigReadError {
    fn description(&self) -> &str {
        &self.description
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // TODO: ???
        None
    }
}

impl fmt::Display for ConfigReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.description)
    }
}

impl fmt::Debug for ConfigReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

