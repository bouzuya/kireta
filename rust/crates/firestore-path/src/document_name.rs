use crate::{CollectionName, DatabaseName, DocumentPath};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database name {0}")]
    DatabaseName(#[from] crate::database_name::Error),
    #[error("document path {0}")]
    DocumentPath(#[from] crate::document_path::Error),
    #[error("todo")]
    ToDo,
}

/// format:
/// - `{database_name}/{document_path}`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DocumentName {
    database_name: DatabaseName,
    document_path: DocumentPath,
}

impl DocumentName {
    pub fn new(database_name: DatabaseName, document_path: DocumentPath) -> Self {
        Self {
            database_name,
            document_path,
        }
    }

    pub fn collection(self, collection_id: &str) -> Result<CollectionName, Error> {
        Ok(CollectionName::new(
            self.database_name,
            self.document_path.collection(collection_id)?,
        ))
    }
}

impl std::fmt::Display for DocumentName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.database_name, self.document_path)
    }
}

impl std::str::FromStr for DocumentName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // <https://firebase.google.com/docs/firestore/quotas#collections_documents_and_fields>
        if s.len() > 6_144 {
            return Err(Error::ToDo);
        }

        let parts = s.split('/').collect::<Vec<&str>>();
        if parts.len() < 5 + 2 || (parts.len() - 5) % 2 != 0 {
            return Err(Error::ToDo);
        }

        Ok(Self {
            database_name: DatabaseName::from_str(&parts[0..5].join("/"))?,
            document_path: DocumentPath::from_str(&parts[5..].join("/"))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{CollectionPath, DatabaseId, DocumentId, ProjectId};

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1";
        let document_name = DocumentName::from_str(s)?;
        assert_eq!(document_name.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_collection() -> anyhow::Result<()> {
        let document_name = DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1",
        )?;
        let collection_name = document_name.collection("messages")?;
        assert_eq!(
            collection_name,
            CollectionName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages"
            )?
        );
        let document_name = DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1",
        )?;
        let collection_name = document_name.collection("col")?;
        assert_eq!(
            collection_name,
            CollectionName::from_str(
                "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages/message1/col"
            )?
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_str() -> anyhow::Result<()> {
        assert!(
            DocumentName::from_str("projects/my-project/databases/my-database/documents").is_err()
        );
        assert!(
            DocumentName::from_str("projects/my-project/databases/my-database/documents/c")
                .is_err()
        );
        assert!(
            DocumentName::from_str("projects/my-project/databases/my-database/documents/c/d")
                .is_ok()
        );
        assert!(DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/c/d/c"
        )
        .is_err());
        assert!(DocumentName::from_str(
            "projects/my-project/databases/my-database/documents/c/d/c/d"
        )
        .is_ok());

        let b = "projects/my-project/databases/my-database/documents";
        let c1 = "x".repeat(1500);
        let d1 = "x".repeat(1500);
        let c2 = "y".repeat(1500);
        let d2 = "y".repeat(1500);
        let c3 = "z".repeat(80);
        let d3_ok = "z".repeat(7);
        let d3_err = "z".repeat(7 + 1);
        let s = format!("{}/{}/{}/{}/{}/{}/{}", b, c1, d1, c2, d2, c3, d3_ok);
        assert_eq!(s.len(), 6_144);
        assert!(DocumentName::from_str(&s).is_ok());
        let s = format!("{}/{}/{}/{}/{}/{}/{}", b, c1, d1, c2, d2, c3, d3_err);
        assert_eq!(s.len(), 6_145);
        assert!(DocumentName::from_str(&s).is_err());
        Ok(())
    }

    #[test]
    fn test_new() -> anyhow::Result<()> {
        let database_name = build_database_name()?;
        let document_path = build_document_path()?;
        let document_name = DocumentName::new(database_name.clone(), document_path.clone());
        assert_eq!(
            document_name.to_string(),
            format!("{}/{}", database_name, document_path)
        );
        Ok(())
    }

    fn build_document_path() -> anyhow::Result<DocumentPath> {
        let collection_path = CollectionPath::from_str("chatrooms")?;
        let document_id = DocumentId::from_str("chatroom1")?;
        let document_path = DocumentPath::new(collection_path, document_id);
        Ok(document_path)
    }

    fn build_database_name() -> anyhow::Result<DatabaseName> {
        let project_id = ProjectId::from_str("my-project")?;
        let database_id = DatabaseId::from_str("my-database")?;
        let database_name = DatabaseName::new(project_id, database_id);
        Ok(database_name)
    }
}
