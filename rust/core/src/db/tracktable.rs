use chrono::DateTime;
use chrono::Utc;
use postgres_types::{FromSql, ToSql};
use uuid::Uuid;

#[derive(Debug, ToSql, FromSql)]
pub struct Col {
    pub(crate) name: String,
    pub(crate) data_type: String,
}

#[allow(dead_code)]
pub struct TrackTable {
    id: Uuid,
    pub(crate) name: String,
    pub on: On,
    pub(crate) p_id: Option<Col>,
    table: String,
    pub(crate) track: Vec<Col>,
    pub(crate) old: bool,
    pub(crate) new: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
impl TrackTable {
    pub fn new<T: IntoIterator<Item = Col>>(
        id: Uuid,
        name: String,
        on: On,
        p_id: Option<Col>,
        table: String,
        track: T,
        old: bool,
        new: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        TrackTable {
            id,
            name,
            on,
            p_id,
            table,
            track: track.into_iter().collect(),
            old,
            new,
            created_at,
            updated_at,
        }
    }

    pub fn table_name(&self) -> String {
        format!("liveo_tracktable_{}", self.name)
    }

    pub fn func_name(&self) -> String {
        format!("liveo_func_{}", self.name)
    }

    pub fn trigger_name(&self) -> String {
        format!("liveo_trigger_{}", self.name)
    }
}

#[allow(dead_code)]
pub enum On {
    Insert,
    Update,
    Delete,
}

impl On {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "insert" => Some(On::Insert),
            "update" => Some(On::Update),
            "delete" => Some(On::Delete),
            _ => None,
        }
    }
}

impl std::fmt::Display for On {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            On::Insert => write!(f, "insert"),
            On::Update => write!(f, "update"),
            On::Delete => write!(f, "delete"),
        }
    }
}
