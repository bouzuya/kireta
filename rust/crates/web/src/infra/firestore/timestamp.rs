#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Timestamp {
    seconds: i64,
    nanos: i32,
}

impl From<prost_types::Timestamp> for Timestamp {
    fn from(prost_types::Timestamp { seconds, nanos }: prost_types::Timestamp) -> Self {
        Self { seconds, nanos }
    }
}

impl From<Timestamp> for prost_types::Timestamp {
    fn from(Timestamp { seconds, nanos }: Timestamp) -> Self {
        Self { seconds, nanos }
    }
}
