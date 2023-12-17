use crate::{CollectionPath, DatabaseName};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("collection path {0}")]
    CollectionPath(#[from] crate::collection_path::Error),
    #[error("database name {0}")]
    DatabaseName(#[from] crate::database_name::Error),
    #[error("todo")]
    ToDo,
}

/// format:
/// - `{database_name}/{collection_path}`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CollectionName {
    collection_path: CollectionPath,
    database_name: DatabaseName,
}

impl CollectionName {
    pub fn new(database_name: DatabaseName, collection_path: CollectionPath) -> Self {
        Self {
            collection_path,
            database_name,
        }
    }
}

impl std::fmt::Display for CollectionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.database_name, self.collection_path)
    }
}

impl std::str::FromStr for CollectionName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // <https://firebase.google.com/docs/firestore/quotas#collections_documents_and_fields>
        if s.len() > 6_144 {
            return Err(Error::ToDo);
        }

        let parts = s.split('/').collect::<Vec<&str>>();
        if parts.len() < 5 + 1 || (parts.len() - 5) % 2 == 0 {
            return Err(Error::ToDo);
        }

        Ok(Self {
            collection_path: CollectionPath::from_str(&parts[5..].join("/"))?,
            database_name: DatabaseName::from_str(&parts[0..5].join("/"))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{CollectionId, DatabaseId, ProjectId};

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "projects/my-project/databases/my-database/documents/chatrooms";
        let collection_path = CollectionName::from_str(s)?;
        assert_eq!(collection_path.to_string(), s);

        let s = "projects/my-project/databases/my-database/documents/chatrooms/chatroom1/messages";
        let collection_path = CollectionName::from_str(s)?;
        assert_eq!(collection_path.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_impl_from_str() -> anyhow::Result<()> {
        assert!(
            CollectionName::from_str("projects/my-project/databases/my-database/documents")
                .is_err()
        );
        assert!(
            CollectionName::from_str("projects/my-project/databases/my-database/documents/c")
                .is_ok()
        );
        assert!(CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/c/d"
        )
        .is_err());
        assert!(CollectionName::from_str(
            "projects/my-project/databases/my-database/documents/c/d/c"
        )
        .is_ok());

        let b = "projects/my-project/databases/my-database/documents";
        let c1 = "x".repeat(1500);
        let d1 = "x".repeat(1500);
        let c2 = "y".repeat(1500);
        let d2 = "y".repeat(1500);
        let c3_ok = "z".repeat(88);
        let c3_err = "z".repeat(88 + 1);
        let s = format!("{}/{}/{}/{}/{}/{}", b, c1, d1, c2, d2, c3_ok);
        assert_eq!(s.len(), 6_144);
        assert!(CollectionName::from_str(&s).is_ok());
        let s = format!("{}/{}/{}/{}/{}/{}", b, c1, d1, c2, d2, c3_err);
        assert_eq!(s.len(), 6_145);
        assert!(CollectionName::from_str(&s).is_err());
        Ok(())
    }

    #[test]
    fn test_new() -> anyhow::Result<()> {
        let database_name = build_database_name()?;
        let collection_path = build_collection_path()?;
        let collection_name = CollectionName::new(database_name.clone(), collection_path.clone());
        assert_eq!(
            collection_name.to_string(),
            format!("{}/{}", database_name, collection_path)
        );
        Ok(())
    }

    fn build_collection_path() -> anyhow::Result<CollectionPath> {
        let collection_id = CollectionId::from_str("chatrooms")?;
        let collection_path = CollectionPath::new(None, collection_id);
        Ok(collection_path)
    }

    fn build_database_name() -> anyhow::Result<DatabaseName> {
        let project_id = ProjectId::from_str("my-project")?;
        let database_id = DatabaseId::from_str("my-database")?;
        let database_name = DatabaseName::new(project_id, database_id);
        Ok(database_name)
    }
}
