pub mod client;
pub mod document;
pub mod path;
pub mod timestamp;

#[cfg(test)]
mod tests {
    use crate::infra::firestore::{client::Client, document::Document, path::RootPath};

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let endpoint = "http://firebase:8080";
        let root_path = RootPath::new("demo-project1".to_string(), "(default)".to_string())?;
        let collection_path = root_path.collection("repositories".to_string());

        let mut client = Client::new(endpoint).await?;

        // reset
        let (documents, _) = client.list::<V>(&collection_path).await?;
        for doc in documents {
            client.delete(doc.name(), doc.update_time()).await?;
        }

        // CREATE
        #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
        struct V {
            k1: String,
        }
        let input = V {
            k1: "v1".to_string(),
        };
        let created = client.create(&collection_path, input.clone()).await?;
        assert!(created
            .name()
            .path()
            .starts_with("projects/demo-project1/databases/(default)/documents/repositories/"),);
        assert_eq!(created.clone().data(), input);

        // READ (GET)
        let got = client.get(created.name()).await?;
        assert_eq!(got, created);

        // READ (LIST)
        let (documents, next_page_token) = client.list::<V>(&collection_path).await?;
        assert_eq!(documents, vec![got.clone()]);
        assert_eq!(next_page_token, "");

        // UPDATE
        let updated: Document<V> = client
            .update(
                got.name(),
                V {
                    k1: "v2".to_owned(), // "v1" -> "v2
                },
                got.update_time(),
            )
            .await?;
        assert_eq!(
            updated.clone().data(),
            V {
                k1: "v2".to_string()
            }
        );

        // DELETE
        client.delete(updated.name(), updated.update_time()).await?;

        Ok(())
    }
}
