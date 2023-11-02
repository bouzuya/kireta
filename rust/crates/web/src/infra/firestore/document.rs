use google_api_proto::google::firestore::v1::{
    value::ValueType, Document as FirestoreDocument, MapValue, Value,
};
use prost_types::Timestamp;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("create_time is none")]
    CreateTimeIsNone,
    #[error("update_time is none")]
    UpdateTimeIsNone,
    #[error("deserialize")]
    Deserialize(#[from] serde_firestore_value::Error),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Document<T> {
    create_time: Timestamp,
    data: T,
    name: String,
    update_time: Timestamp,
}

impl<T: serde::de::DeserializeOwned> Document<T> {
    pub fn new(
        FirestoreDocument {
            create_time,
            fields,
            name,
            update_time,
        }: FirestoreDocument,
    ) -> Result<Self, Error> {
        let create_time = create_time.ok_or(Error::CreateTimeIsNone)?;
        let data: T = serde_firestore_value::from_value(&Value {
            value_type: Some(ValueType::MapValue(MapValue { fields })),
        })?;
        let update_time = update_time.ok_or(Error::UpdateTimeIsNone)?;
        Ok(Self {
            create_time,
            data,
            name,
            update_time,
        })
    }

    pub fn create_time(self) -> Timestamp {
        self.create_time
    }

    pub fn data(self) -> T {
        self.data
    }

    pub fn name(self) -> String {
        self.name
    }

    pub fn update_time(self) -> Timestamp {
        self.update_time
    }
}
