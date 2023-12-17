#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("todo")]
    ToDo,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProjectId(String);

impl std::fmt::Display for ProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for ProjectId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // <https://cloud.google.com/resource-manager/docs/creating-managing-projects>

        if !(6..=30).contains(&s.len()) {
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
        if last_char == '-' {
            return Err(Error::ToDo);
        }

        if s.contains("google")
            || s.contains("null")
            || s.contains("undefined")
            || s.contains("ssl")
        {
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
        let s = "my-project";
        let project_id = ProjectId::from_str(s)?;
        assert_eq!(project_id.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_impl_from_str() -> anyhow::Result<()> {
        assert!(ProjectId::from_str(&"x".repeat(5)).is_err());
        assert!(ProjectId::from_str(&"x".repeat(6)).is_ok());
        assert!(ProjectId::from_str(&"x".repeat(30)).is_ok());
        assert!(ProjectId::from_str(&"x".repeat(31)).is_err());
        assert!(ProjectId::from_str("chat/rooms").is_err());
        assert!(ProjectId::from_str("xxxxxx").is_ok());
        assert!(ProjectId::from_str("x-xxxx").is_ok());
        assert!(ProjectId::from_str("x0xxxx").is_ok());
        assert!(ProjectId::from_str("xAxxxx").is_err());
        assert!(ProjectId::from_str("0xxxxx").is_err());
        assert!(ProjectId::from_str("xxxxx0").is_ok());
        assert!(ProjectId::from_str("xxxxx-").is_err());
        assert!(ProjectId::from_str("xgoogle").is_err());
        assert!(ProjectId::from_str("xnull").is_err());
        assert!(ProjectId::from_str("xundefined").is_err());
        assert!(ProjectId::from_str("xssl").is_err());
        Ok(())
    }
}
