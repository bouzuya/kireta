pub mod client;
pub mod document;
pub mod path;
pub mod timestamp;

#[cfg(test)]
mod tests {
    use crate::infra::firestore::{
        client::{Client, Transaction},
        document::Document,
        path::DocumentPath,
    };

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let endpoint = "http://firebase:8080";
        let mut client = Client::new(
            "demo-project1".to_string(),
            "(default)".to_string(),
            endpoint,
        )
        .await?;
        let collection_path = client.collection("repositories".to_string());

        assert_eq!(
            collection_path.path(),
            "projects/demo-project1/databases/(default)/documents/repositories"
        );

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
        let document_path = collection_path.clone().doc("1".to_string());
        let created = client.create(&document_path, input.clone()).await?;
        assert_eq!(
            created.name().path(),
            "projects/demo-project1/databases/(default)/documents/repositories/1"
        );
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

    #[tokio::test]
    async fn test_transaction() -> anyhow::Result<()> {
        let endpoint = "http://firebase:8080";
        let mut client = Client::new(
            "demo-project1".to_string(),
            "(default)".to_string(),
            endpoint,
        )
        .await?;
        let collection_path = client.collection("transactions".to_string());

        // reset
        let (documents, _) = client.list::<V>(&collection_path).await?;
        for doc in documents {
            client.delete(doc.name(), doc.update_time()).await?;
        }

        let document_path = collection_path.doc("1".to_string());

        #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
        struct V {
            k1: String,
        }

        let input = V {
            k1: "v1".to_string(),
        };
        let mut transaction = client.begin_transaction().await?;
        transaction.create(&document_path, input)?;
        transaction.commit().await?;
        assert!(client.get::<V>(&document_path).await.is_ok());

        async fn f(client: &mut Client, document_path: &DocumentPath) -> anyhow::Result<()> {
            let mut transaction = client.begin_transaction().await?;
            let got = transaction.get::<V>(document_path).await?;
            transaction.delete(document_path, got.update_time())?;
            transaction.rollback().await?;
            Ok(())
        }
        f(&mut client, &document_path).await?;
        // Not deleted because it was rolled back
        assert!(client.get::<V>(&document_path).await.is_ok());

        let got = client.get::<V>(&document_path).await?;
        let current_update_time = got.update_time();
        let mut transaction = client.begin_transaction().await?;
        transaction.delete(&document_path, current_update_time)?;
        transaction.commit().await?;
        let err = client.get::<V>(&document_path).await.unwrap_err();
        if let crate::infra::firestore::client::Error::Status(status) = err {
            assert_eq!(status.code(), tonic::Code::NotFound);
        } else {
            panic!("unexpected error: {:?}", err);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_run_transaction() -> anyhow::Result<()> {
        let endpoint = "http://firebase:8080";
        let mut client = Client::new(
            "demo-project1".to_string(),
            "(default)".to_string(),
            endpoint,
        )
        .await?;
        let collection_path = client.collection("transactions".to_string());

        // reset
        let (documents, _) = client.list::<V>(&collection_path).await?;
        for doc in documents {
            client.delete(doc.name(), doc.update_time()).await?;
        }

        let document_path = collection_path.doc("1".to_string());

        #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
        struct V {
            k1: String,
        }

        client
            .run_transaction(|transaction: &mut Transaction| {
                let p = document_path.clone();
                Box::pin(async move {
                    let input = V {
                        k1: "v1".to_string(),
                    };
                    transaction.create(&p, input)?;
                    Ok(())
                })
            })
            .await?;
        assert!(client.get::<V>(&document_path).await.is_ok());

        // async fn f(client: &mut Client, document_path: &DocumentPath) -> anyhow::Result<()> {
        //     let mut transaction = client.begin_transaction().await?;
        //     let got = transaction.get::<V>(document_path).await?;
        //     transaction.delete(document_path, got.update_time())?;
        //     transaction.rollback().await?;
        //     Ok(())
        // }
        // f(&mut client, &document_path).await?;
        // // Not deleted because it was rolled back
        // assert!(client.get::<V>(&document_path).await.is_ok());

        // let got = client.get::<V>(&document_path).await?;
        // let current_update_time = got.update_time();
        // let mut transaction = client.begin_transaction().await?;
        // transaction.delete(&document_path, current_update_time)?;
        // transaction.commit().await?;
        // let err = client.get::<V>(&document_path).await.unwrap_err();
        // if let crate::infra::firestore::client::Error::Status(status) = err {
        //     assert_eq!(status.code(), tonic::Code::NotFound);
        // } else {
        //     panic!("unexpected error: {:?}", err);
        // }

        Ok(())
    }
}
