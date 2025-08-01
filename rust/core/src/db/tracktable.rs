use super::queries;
use super::utils;
use std::error::Error;
use tokio_postgres::Client;

#[allow(dead_code)]
pub struct TrackTable {
    pub on: On,
    pub table: String,
    pub track_col: Vec<String>,
    pub old: bool,
    pub new: bool,
}

#[allow(dead_code)]
impl TrackTable {
    fn new<T: IntoIterator<Item = String>>(
        on: On,
        table: String,
        track: T,
        old: bool,
        new: bool,
    ) -> Self {
        TrackTable {
            on,
            table,
            track_col: track.into_iter().collect(),
            old,
            new,
        }
    }

    async fn create_table(&self, client: &Client) -> Result<(), Box<dyn Error>> {
        // Validate identifiers first
        utils::is_valid_identifier(&self.table, "Invalid table name")?;

        for col in &self.track_col {
            utils::is_valid_identifier(col, "Invalid column name")?;
        }

        // Use the queries module to generate the SQL dynamically
        let query = queries::create_tracking_table_sql(self);

        client.execute(&query, &[]).await?;
        Ok(())
    }

    async fn create_trigger(&self, client: &Client) -> Result<(), Box<dyn Error>> {
        // Placeholder for database trigger creation logic
        Ok(())
    }

    pub fn table_name(&self) -> String {
        format!(
            "liveo_track_{}_col_{}_on_{}",
            self.table,
            self.track_col.join("_"),
            self.on.to_string()
        )
    }

    pub fn func_name(&self) -> String {
        format!(
            "liveo_track_{}_col_{}_on_{}_func",
            self.table,
            self.track_col.join("_"),
            self.on.to_string()
        )
    }

    pub fn trigger_name(&self) -> String {
        format!(
            "liveo_track_{}_col_{}_on_{}_trigger",
            self.table,
            self.track_col.join("_"),
            self.on.to_string()
        )
    }
}

#[allow(dead_code)]
pub enum On {
    Insert,
    Update,
    Delete,
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
