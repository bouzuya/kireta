#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Path {
    Collection(CollectionPath),
    Document(DocumentPath),
    Root(RootPath),
}

impl Path {
    pub fn path(&self) -> String {
        match self {
            Path::Collection(p) => p.path(),
            Path::Document(p) => p.path(),
            Path::Root(p) => p.path(),
        }
    }

    pub fn root(&self) -> &RootPath {
        match self {
            Path::Collection(p) => p.root(),
            Path::Document(p) => p.root(),
            Path::Root(p) => p,
        }
    }
}

impl From<CollectionPath> for Path {
    fn from(value: CollectionPath) -> Self {
        Self::Collection(value)
    }
}

impl From<DocumentPath> for Path {
    fn from(value: DocumentPath) -> Self {
        Self::Document(value)
    }
}

impl From<RootPath> for Path {
    fn from(value: RootPath) -> Self {
        Self::Root(value)
    }
}

impl std::str::FromStr for Path {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1_024 * 6 {
            anyhow::bail!("too long");
        }

        let parts = s.split('/').collect::<Vec<&str>>();
        if parts.len() < 5
            || parts[0] != "projects"
            || parts[2] != "databases"
            || parts[4] != "documents"
        {
            anyhow::bail!("invalid root path");
        }

        // TODO: check Maximum depth of subcollections (<= 100)

        // TODO: check `"."` and `".."` and `"__.*__"`
        // TODO: check len (<= 1500)
        let mut path = Path::from(RootPath {
            database_id: parts[3].to_string(),
            project_id: parts[1].to_string(),
        });
        for s in parts.into_iter().skip(5).map(|s| s.to_string()) {
            path = match path {
                Path::Collection(p) => Path::from(p.doc(s)),
                Path::Document(p) => Path::from(p.collection(s)),
                Path::Root(p) => Path::from(p.collection(s)),
            };
        }
        Ok(path)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CollectionPath {
    id: String,
    parent: Box<Path>,
}

impl CollectionPath {
    pub fn doc(self, document_id: String) -> DocumentPath {
        DocumentPath {
            id: document_id,
            parent: self,
        }
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn parent(&self) -> &Path {
        self.parent.as_ref()
    }

    pub fn path(&self) -> String {
        format!("{}/{}", self.parent.path(), self.id)
    }

    pub fn root(&self) -> &RootPath {
        self.parent.root()
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DocumentPath {
    id: String,
    parent: CollectionPath,
}

impl DocumentPath {
    pub fn collection(self, collection_id: String) -> CollectionPath {
        CollectionPath {
            id: collection_id,
            parent: Box::new(Path::from(self)),
        }
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn parent(&self) -> &CollectionPath {
        &self.parent
    }

    pub fn path(&self) -> String {
        format!("{}/{}", self.parent.path(), self.id)
    }

    pub fn root(&self) -> &RootPath {
        self.parent.root()
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RootPath {
    database_id: String,
    project_id: String,
}

impl RootPath {
    pub fn collection(self, collection_id: String) -> CollectionPath {
        CollectionPath {
            id: collection_id,
            parent: Box::new(Path::from(self)),
        }
    }

    pub fn database_id(&self) -> &str {
        self.database_id.as_str()
    }

    pub fn path(&self) -> String {
        format!(
            "projects/{}/databases/{}/documents",
            self.project_id, self.database_id
        )
    }

    pub fn project_id(&self) -> &str {
        self.project_id.as_str()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_root_path_from_str() -> anyhow::Result<()> {
        // 6KiB
        let s = format!(
            "{}/{}/{}/{}/{}/{}/{}",
            "projects/demo-project1/databases/(default)/documents",
            "1".repeat(1024),
            "2".repeat(1024),
            "3".repeat(1024),
            "4".repeat(1024),
            "5".repeat(1024),
            "6".repeat(1024 - 58)
        );
        assert_eq!(s.len(), 1_024 * 6);
        assert!(Path::from_str(&s).is_ok());
        let s = format!("{}a", s);
        assert_eq!(s.len(), 1_024 * 6 + 1);
        assert!(Path::from_str(&s).is_err());

        assert!(Path::from_str("projects1/demo-project1/databases/(default)/documents").is_err());
        assert!(Path::from_str("projects/demo-project1/databases1/(default)/documents").is_err());
        assert!(Path::from_str("projects/demo-project1/databases/(default)/documents1").is_err());

        let path = Path::from_str("projects/demo-project1/databases/(default)/documents")?;
        assert_eq!(
            path,
            Path::Root(RootPath {
                database_id: "(default)".to_string(),
                project_id: "demo-project1".to_string(),
            })
        );

        let path = Path::from_str("projects/demo-project1/databases/(default)/documents/users")?;
        assert_eq!(
            path,
            Path::Collection(CollectionPath {
                id: "users".to_string(),
                parent: Box::new(Path::Root(RootPath {
                    database_id: "(default)".to_string(),
                    project_id: "demo-project1".to_string(),
                }))
            })
        );

        let path = Path::from_str("projects/demo-project1/databases/(default)/documents/users/1")?;
        assert_eq!(
            path,
            Path::Document(DocumentPath {
                id: "1".to_string(),
                parent: CollectionPath {
                    id: "users".to_string(),
                    parent: Box::new(Path::Root(RootPath {
                        database_id: "(default)".to_string(),
                        project_id: "demo-project1".to_string(),
                    }))
                }
            })
        );

        let path = Path::from_str(
            "projects/demo-project1/databases/(default)/documents/users/1/repositories",
        )?;
        assert_eq!(
            path,
            Path::Collection(CollectionPath {
                id: "repositories".to_string(),
                parent: Box::new(Path::Document(DocumentPath {
                    id: "1".to_string(),
                    parent: CollectionPath {
                        id: "users".to_string(),
                        parent: Box::new(Path::Root(RootPath {
                            database_id: "(default)".to_string(),
                            project_id: "demo-project1".to_string(),
                        }))
                    }
                }))
            })
        );

        let path = Path::from_str(
            "projects/demo-project1/databases/(default)/documents/users/1/repositories/2",
        )?;
        assert_eq!(
            path,
            Path::Document(DocumentPath {
                id: "2".to_string(),
                parent: CollectionPath {
                    id: "repositories".to_string(),
                    parent: Box::new(Path::Document(DocumentPath {
                        id: "1".to_string(),
                        parent: CollectionPath {
                            id: "users".to_string(),
                            parent: Box::new(Path::Root(RootPath {
                                database_id: "(default)".to_string(),
                                project_id: "demo-project1".to_string(),
                            }))
                        }
                    }))
                }
            })
        );

        Ok(())
    }

    #[test]
    fn test() {
        // root_path
        let root_path = RootPath {
            database_id: "(default)".to_string(),
            project_id: "demo-project1".to_string(),
        };
        assert_eq!(root_path.database_id(), "(default)");
        assert_eq!(
            root_path.path(),
            "projects/demo-project1/databases/(default)/documents"
        );
        assert_eq!(root_path.project_id(), "demo-project1");

        // collection_path
        let collection_path = root_path.collection("users".to_string());
        assert_eq!(collection_path.id(), "users");
        assert_eq!(
            collection_path.parent().path(),
            "projects/demo-project1/databases/(default)/documents"
        );
        assert_eq!(
            collection_path.path(),
            "projects/demo-project1/databases/(default)/documents/users"
        );
        assert_eq!(
            collection_path.root().path(),
            "projects/demo-project1/databases/(default)/documents"
        );

        // document_path
        let document_path = collection_path.doc("1".to_string());
        assert_eq!(document_path.id(), "1");
        assert_eq!(
            document_path.parent().path(),
            "projects/demo-project1/databases/(default)/documents/users"
        );
        assert_eq!(
            document_path.path(),
            "projects/demo-project1/databases/(default)/documents/users/1"
        );
        assert_eq!(
            document_path.root().path(),
            "projects/demo-project1/databases/(default)/documents"
        );

        // collection_path (nested)
        let nested_collection_path = document_path.collection("repositories".to_string());
        assert_eq!(nested_collection_path.id(), "repositories");
        assert_eq!(
            nested_collection_path.parent().path(),
            "projects/demo-project1/databases/(default)/documents/users/1"
        );
        assert_eq!(
            nested_collection_path.path(),
            "projects/demo-project1/databases/(default)/documents/users/1/repositories"
        );
        assert_eq!(
            nested_collection_path.root().path(),
            "projects/demo-project1/databases/(default)/documents"
        );

        // document_path (nested)
        let nested_document_path = nested_collection_path.doc("2".to_string());
        assert_eq!(nested_document_path.id(), "2");
        assert_eq!(
            nested_document_path.parent().path(),
            "projects/demo-project1/databases/(default)/documents/users/1/repositories"
        );
        assert_eq!(
            nested_document_path.path(),
            "projects/demo-project1/databases/(default)/documents/users/1/repositories/2"
        );
        assert_eq!(
            nested_document_path.root().path(),
            "projects/demo-project1/databases/(default)/documents"
        );
    }
}
