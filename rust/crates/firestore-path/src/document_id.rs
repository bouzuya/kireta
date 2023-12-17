#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("todo")]
    ToDo,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DocumentId(String);

impl std::fmt::Display for DocumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for DocumentId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // <https://firebase.google.com/docs/firestore/quotas#collections_documents_and_fields>
        if s.len() > 1_500 {
            return Err(Error::ToDo);
        }

        if s.contains('/') {
            return Err(Error::ToDo);
        }

        if s == "." || s == ".." {
            return Err(Error::ToDo);
        }

        if s.starts_with("__") && s.ends_with("__") {
            return Err(Error::ToDo);
        }

        // TODO: Datastore entities

        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "chatroom1";
        let document_id = DocumentId::from_str(s)?;
        assert_eq!(document_id.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_impl_from_str() -> anyhow::Result<()> {
        assert!(DocumentId::from_str("chatroom1").is_ok());
        assert!(DocumentId::from_str(&"x".repeat(1501)).is_err());
        assert!(DocumentId::from_str(&"x".repeat(1500)).is_ok());
        assert!(DocumentId::from_str("chat/room1").is_err());
        assert!(DocumentId::from_str(".").is_err());
        assert!(DocumentId::from_str(".x").is_ok());
        assert!(DocumentId::from_str("..").is_err());
        assert!(DocumentId::from_str("..x").is_ok());
        assert!(DocumentId::from_str("__x__").is_err());
        assert!(DocumentId::from_str("__x").is_ok());
        assert!(DocumentId::from_str("x__").is_ok());
        Ok(())
    }
}
