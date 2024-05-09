pub mod states;
use std::fmt::Display;

pub use states::State;

#[derive(Debug, Clone)]
pub struct ConversionError;

impl Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Provided value was not an abbreviation or state name of the USA")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(State::California.to_string(), "California");
        assert_eq!(State::NewYork.to_string(), "New York");
    }

    #[test]
    fn test_abbreviation() {
        assert_eq!(State::California.abbreviation(), "CA");
        assert_eq!(State::NewYork.abbreviation(), "NY");
    }

    #[test]
    #[cfg(feature = "serde_abbreviation")]
    fn test_serialize_abbreviation() {
        let state = State::California;
        let serialized = serde_json::to_string(&state).unwrap();
        assert_eq!(serialized, "\"CA\"");
    }

    #[test]
    #[cfg(all(feature = "serde1", not(feature = "serde_abbreviation")))]
    fn test_serialize_display() {
        let state = State::California;
        let serialized = serde_json::to_string(&state).unwrap();
        assert_eq!(serialized, "\"California\"");
    }

    #[test]
    #[cfg(feature = "serde_abbreviation")]
    fn test_deserialize_abbreviation() {
        let deserialized: State = serde_json::from_str("\"CA\"").unwrap();
        assert_eq!(deserialized, State::California);
    }

    #[test]
    #[cfg(all(feature = "serde1", not(feature = "serde_abbreviation")))]
    fn test_deserialize_display() {
        let deserialized: State = serde_json::from_str("\"California\"").unwrap();
        assert_eq!(deserialized, State::California);
    }
}
