#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("todo")]
    ToDo,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DatabaseId(String);

impl std::fmt::Display for DatabaseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for DatabaseId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // <https://firebase.google.com/docs/firestore/reference/rest/v1/projects.databases/create#query-parameters>
        if s == "(default)" {
            return Ok(Self(s.to_string()));
        }

        if !(4..=63).contains(&s.len()) {
            return Err(Error::ToDo);
        }

        if !s
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(Error::ToDo);
        }

        let first_char = s.chars().next().expect("already length checked");
        if !first_char.is_ascii_lowercase() {
            return Err(Error::ToDo);
        }

        let last_char = s.chars().next_back().expect("already length checked");
        if !(last_char.is_ascii_lowercase() || last_char.is_ascii_digit()) {
            return Err(Error::ToDo);
        }

        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "my-database";
        let database_id = DatabaseId::from_str(s)?;
        assert_eq!(database_id.to_string(), s);

        let s = "(default)";
        let database_id = DatabaseId::from_str(s)?;
        assert_eq!(database_id.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_impl_from_str() -> anyhow::Result<()> {
        assert!(DatabaseId::from_str("(default)").is_ok());
        assert!(DatabaseId::from_str("(default1)").is_err());
        assert!(DatabaseId::from_str(&"x".repeat(3)).is_err());
        assert!(DatabaseId::from_str(&"x".repeat(4)).is_ok());
        assert!(DatabaseId::from_str(&"x".repeat(63)).is_ok());
        assert!(DatabaseId::from_str(&"x".repeat(64)).is_err());
        assert!(DatabaseId::from_str("x1-x").is_ok());
        assert!(DatabaseId::from_str("xAxx").is_err());
        assert!(DatabaseId::from_str("-xxx").is_err());
        assert!(DatabaseId::from_str("0xxx").is_err());
        assert!(DatabaseId::from_str("xxx-").is_err());
        assert!(DatabaseId::from_str("xxx0").is_ok());
        Ok(())
    }
}
