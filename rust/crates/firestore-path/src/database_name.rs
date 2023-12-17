use crate::{DatabaseId, ProjectId};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database id {0}")]
    DatabaseId(#[from] crate::database_id::Error),
    #[error("project id {0}")]
    ProjectId(#[from] crate::project_id::Error),
    #[error("todo")]
    ToDo,
}

// format: `projects/{project_id}/databases/{database_id}/documents`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DatabaseName {
    database_id: DatabaseId,
    project_id: ProjectId,
}

impl DatabaseName {
    pub fn new(project_id: ProjectId, database_id: DatabaseId) -> Self {
        Self {
            database_id,
            project_id,
        }
    }
}

impl std::fmt::Display for DatabaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "projects/{}/databases/{}/documents",
            self.project_id, self.database_id
        )
    }
}

impl std::str::FromStr for DatabaseName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1_024 * 6 {
            return Err(Error::ToDo);
        }

        let parts = s.split('/').collect::<Vec<&str>>();
        if parts.len() != 5
            || parts[0] != "projects"
            || parts[2] != "databases"
            || parts[4] != "documents"
        {
            return Err(Error::ToDo);
        }

        let project_id = ProjectId::from_str(parts[1])?;
        let database_id = DatabaseId::from_str(parts[3])?;
        Ok(Self {
            database_id,
            project_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let s = "projects/my-project/databases/my-database/documents";
        let database_name = DatabaseName::from_str(s)?;
        assert_eq!(database_name.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_impl_from_str() -> anyhow::Result<()> {
        let s = "projects/my-project/databases/my-database/documents";
        let database_name = DatabaseName::from_str(s)?;
        assert_eq!(database_name.to_string(), s);
        assert!(DatabaseName::from_str(&"x".repeat(1024 * 6 + 1)).is_err());
        assert!(DatabaseName::from_str("p/my-project/databases/my-database/documents").is_err());
        assert!(DatabaseName::from_str("projects/my-project/d/my-database/documents").is_err());
        assert!(DatabaseName::from_str("projects/my-project/databases/my-database/d").is_err());
        assert!(DatabaseName::from_str("projects/P/databases/my-database/d").is_err());
        assert!(DatabaseName::from_str("projects/my-project/databases/D/d").is_err());
        Ok(())
    }

    #[test]
    fn test_new() -> anyhow::Result<()> {
        let project_id = build_project_id()?;
        let database_id = build_database_id()?;
        let database_name = DatabaseName::new(project_id.clone(), database_id.clone());
        assert_eq!(
            database_name.to_string(),
            format!(
                "projects/{}/databases/{}/documents",
                project_id, database_id
            )
        );
        Ok(())
    }

    fn build_database_id() -> anyhow::Result<DatabaseId> {
        Ok(DatabaseId::from_str("my-database")?)
    }

    fn build_project_id() -> anyhow::Result<ProjectId> {
        Ok(ProjectId::from_str("my-project")?)
    }
}
