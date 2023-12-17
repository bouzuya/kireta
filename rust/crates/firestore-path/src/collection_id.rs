#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("todo")]
    ToDo,
}

/// limit: <https://firebase.google.com/docs/firestore/quotas#collections_documents_and_fields>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CollectionId(String);

impl std::fmt::Display for CollectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for CollectionId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // <https://firebase.google.com/docs/firestore/quotas#collections_documents_and_fields>
        if s.len() > 1500 {
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
        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "chatrooms";
        let collection_id = CollectionId::from_str(s)?;
        assert_eq!(collection_id.to_string(), s);

        let s = "messages";
        let collection_id = CollectionId::from_str(s)?;
        assert_eq!(collection_id.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_impl_from_str() -> anyhow::Result<()> {
        assert!(CollectionId::from_str(&"x".repeat(1501)).is_err());
        assert!(CollectionId::from_str(&"x".repeat(1500)).is_ok());
        assert!(CollectionId::from_str("chat/rooms").is_err());
        assert!(CollectionId::from_str(".").is_err());
        assert!(CollectionId::from_str(".x").is_ok());
        assert!(CollectionId::from_str("..").is_err());
        assert!(CollectionId::from_str("..x").is_ok());
        assert!(CollectionId::from_str("__x__").is_err());
        assert!(CollectionId::from_str("__x").is_ok());
        assert!(CollectionId::from_str("x__").is_ok());
        Ok(())
    }
}
