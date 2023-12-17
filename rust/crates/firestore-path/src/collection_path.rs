use std::str::FromStr;

use crate::{CollectionId, DocumentId, DocumentPath};

#[derive(Debug, thiserror::Error)]
#[error("error")]
pub enum Error {
    #[error("collection id {0}")]
    CollectionId(#[from] crate::collection_id::Error),
    #[error("document id {0}")]
    DocumentId(#[from] crate::document_id::Error),
    #[error("document path {0}")]
    DocumentPath(#[from] crate::document_path::Error),
    #[error("todo")]
    ToDo,
}

/// format:
/// - `{collection_id}`
/// - `{document_path}/{collection_id}`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CollectionPath {
    document_path: Option<DocumentPath>,
    collection_id: CollectionId,
}

impl CollectionPath {
    pub fn new(parent: Option<DocumentPath>, collection_id: CollectionId) -> Self {
        Self {
            document_path: parent,
            collection_id,
        }
    }

    pub fn doc(self, document_id: &str) -> Result<DocumentPath, Error> {
        let document_id = DocumentId::from_str(document_id)?;
        let document_path = DocumentPath::new(self, document_id);
        Ok(document_path)
    }
}

impl std::convert::TryFrom<&str> for CollectionPath {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from(s.to_string())
    }
}

impl std::convert::TryFrom<String> for CollectionPath {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(match s.rsplit_once('/') {
            Some((document_path, collection_id)) => Self {
                document_path: Some(DocumentPath::from_str(document_path)?),
                collection_id: CollectionId::from_str(collection_id)?,
            },
            None => Self {
                document_path: None,
                collection_id: CollectionId::try_from(s)?,
            },
        })
    }
}

impl std::fmt::Display for CollectionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.document_path.as_ref() {
            Some(document_path) => write!(f, "{}/{}", document_path, self.collection_id),
            None => self.collection_id.fmt(f),
        }
    }
}

impl std::str::FromStr for CollectionPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "chatrooms";
        let collection_path = CollectionPath::from_str(s)?;
        assert_eq!(collection_path.to_string(), s);

        let s = "chatrooms/chatroom1/messages";
        let collection_path = CollectionPath::from_str(s)?;
        assert_eq!(collection_path.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_doc() -> anyhow::Result<()> {
        let collection_path = CollectionPath::from_str("chatrooms")?;
        let document_path = collection_path.doc("chatroom1")?;
        assert_eq!(
            document_path,
            DocumentPath::from_str("chatrooms/chatroom1")?
        );

        let collection_path = CollectionPath::from_str("chatrooms/chatroom1/messages")?;
        let document_path = collection_path.doc("message1")?;
        assert_eq!(
            document_path,
            DocumentPath::from_str("chatrooms/chatroom1/messages/message1")?
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_str_and_impl_try_from_string() -> anyhow::Result<()> {
        for (s, expected) in [("chatrooms", true), ("chatrooms/chatroom1/messages", true)] {
            assert_eq!(CollectionPath::from_str(s).is_ok(), expected);
            assert_eq!(CollectionPath::try_from(s).is_ok(), expected);
            assert_eq!(CollectionPath::try_from(s.to_string()).is_ok(), expected);
            if expected {
                assert_eq!(CollectionPath::from_str(s)?, CollectionPath::try_from(s)?);
                assert_eq!(
                    CollectionPath::from_str(s)?,
                    CollectionPath::try_from(s.to_string())?
                );
                assert_eq!(CollectionPath::from_str(s)?.to_string(), s);
            }
        }
        Ok(())
    }

    #[test]
    fn test_new() -> anyhow::Result<()> {
        let collection_id = build_collection_id()?;
        let collection_path = CollectionPath::new(None, collection_id.clone());
        assert_eq!(collection_path.to_string(), format!("{}", collection_id));

        let document_path = build_document_path()?;
        let collection_path =
            CollectionPath::new(Some(document_path.clone()), collection_id.clone());
        assert_eq!(
            collection_path.to_string(),
            format!("{}/{}", document_path, collection_id)
        );
        Ok(())
    }

    fn build_collection_id() -> anyhow::Result<CollectionId> {
        Ok(CollectionId::from_str("chatrooms")?)
    }

    fn build_document_path() -> anyhow::Result<DocumentPath> {
        Ok(DocumentPath::from_str("chatrooms/chatroom1")?)
    }
}
