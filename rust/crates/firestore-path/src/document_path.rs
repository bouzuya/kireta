use crate::{CollectionPath, DocumentId};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("collection path {0}")]
    CollectionPath(#[from] Box<crate::collection_path::Error>),
    #[error("document id {0}")]
    DocumentId(#[from] crate::document_id::Error),
    #[error("todo")]
    ToDo,
}

/// format: `{collection_path}/{document_id}`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DocumentPath {
    collection_path: Box<CollectionPath>,
    document_id: DocumentId,
}

impl DocumentPath {
    pub fn new(collection_path: CollectionPath, document_id: DocumentId) -> Self {
        Self {
            collection_path: Box::new(collection_path),
            document_id,
        }
    }
}

impl std::fmt::Display for DocumentPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.collection_path, self.document_id)
    }
}

impl std::str::FromStr for DocumentPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.rsplit_once('/') {
            Some((collection_path, document_id)) => Self {
                collection_path: Box::new(
                    CollectionPath::from_str(collection_path).map_err(Box::new)?,
                ),
                document_id: DocumentId::from_str(document_id)?,
            },
            None => {
                return Err(Error::ToDo);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "chatrooms/chatroom1";
        let document_path = DocumentPath::from_str(s)?;
        assert_eq!(document_path.to_string(), s);

        let s = "chatrooms/chatroom1/messages/message1";
        let document_path = DocumentPath::from_str(s)?;
        assert_eq!(document_path.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_impl_from_str() -> anyhow::Result<()> {
        let s = "chatrooms";
        assert!(DocumentPath::from_str(s).is_err());

        let s = "chatrooms/chatroom1";
        let document_path = DocumentPath::from_str(s)?;
        assert_eq!(document_path.to_string(), s);

        let s = "chatrooms/chatroom1/messages/message1";
        let document_path = DocumentPath::from_str(s)?;
        assert_eq!(document_path.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_new() -> anyhow::Result<()> {
        let collection_path = build_collection_path()?;
        let document_id = build_document_id()?;
        let document_path = DocumentPath::new(collection_path.clone(), document_id.clone());
        assert_eq!(
            document_path.to_string(),
            format!("{}/{}", collection_path, document_id)
        );
        Ok(())
    }

    fn build_collection_path() -> anyhow::Result<CollectionPath> {
        Ok(CollectionPath::from_str("chatrooms")?)
    }

    fn build_document_id() -> anyhow::Result<DocumentId> {
        Ok(DocumentId::from_str("chatroom1")?)
    }
}
