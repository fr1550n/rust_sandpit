use std::fmt;
use std::fs;
use std::error::Error;
use std::num::ParseFloatError;

// Custom Result and Error types
type Result<T> = std::result::Result<T, TemperatureReadingError>;

#[derive(Debug, Clone)]
pub struct TemperatureReadingError{
    details: String
}

impl TemperatureReadingError {
    fn new(msg: &str) -> TemperatureReadingError {
        TemperatureReadingError { details: msg.to_string() }
    }
}

impl Error for TemperatureReadingError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for TemperatureReadingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "nope: {}", self.details)
    }
}

impl From<ParseFloatError> for TemperatureReadingError {
    fn from(err: ParseFloatError) -> Self {
        TemperatureReadingError::new(&err.to_string())
    }
}

impl From<std::io::Error> for TemperatureReadingError {
    fn from(err: std::io::Error) -> Self {
        TemperatureReadingError::new(&err.to_string())
    }
}
// 

// our custom Result type handles the errors: std::io::Error, std::num::ParseFloatError
// by wrapping them in TemperatureReadingError
pub fn read_temp() -> Result<f32> {
    let cpu_path     = "/sys/class/thermal/thermal_zone0/temp";
    let mut contents = fs::read_to_string(cpu_path)?;
    contents         = contents.trim_end().to_string();
    return Ok(contents.parse::<f32>()? / 1000.)
}