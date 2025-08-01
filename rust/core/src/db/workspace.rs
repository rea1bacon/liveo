use super::tracktable::Col;
use super::tracktable::TrackTable;
use crate::db::queries;
use crate::error::Error as LiveoError;
use chrono::DateTime;
use chrono::Utc;
use tokio_postgres::{Client, Error};
use uuid::Uuid;

pub struct Workspace {
    track_tables: Vec<TrackTable>,
    operations: Vec<Operation>,
}

impl Workspace {
    pub async fn new(client: &Client) -> Result<Self, Error> {
        // Create the col type
        Self::_create_col_type(client).await?;
        // create the workspace table if it doesn't exist
        Self::_create_workspace_table(client).await?;
        // fetch the existing track tables
        let stmt = client.prepare(&queries::fetch_track_tables_sql()).await?;
        let track_tables_result = client.query(&stmt, &[]).await?;
        // format the results into TrackTable instances
        let mut track_tables = Vec::new();
        for row in track_tables_result {
            let id = Uuid::parse_str(&row.get::<_, String>("id")).unwrap();
            let name: String = row.get("name");
            let on: String = row.get("on");
            let table: String = row.get("table");
            let track_col: Vec<Col> = row.get("track");
            let old: bool = row.get("old");
            let new: bool = row.get("new");
            // pid can be null or string, so we handle it accordingly
            let p_id: Option<Col> = row.get("p_id");
            let created_at: DateTime<Utc> = row.get("created_at");
            let updated_at: DateTime<Utc> = row.get("updated_at");

            let track_table = TrackTable::new(
                id,
                name,
                super::tracktable::On::from_str(&on).unwrap(),
                p_id,
                table,
                track_col,
                old,
                new,
                created_at,
                updated_at,
            );
            track_tables.push(track_table);
        }
        Ok(Workspace {
            track_tables,
            operations: Vec::new(),
        })
    }

    async fn _create_workspace_table(client: &Client) -> Result<(), Error> {
        // create the workspace table if it doesn't exist
        let query = queries::create_workspace_table_sql();
        client.execute(&query, &[]).await?;
        Ok(())
    }

    async fn _create_col_type(client: &Client) -> Result<(), Error> {
        // create the col type if it doesn't exist
        let query = queries::create_col_type();
        client.execute(&query, &[]).await?;
        Ok(())
    }

    pub fn add_track_table(&mut self, track_table: TrackTable) -> Result<(), LiveoError> {
        // Check if the track table already exists
        if self
            .track_tables
            .iter()
            .any(|t| t.table_name() == track_table.table_name())
        {
            return Err(LiveoError::new(
                "Adding new track table",
                Some("TrackTable already exists".into()),
            ));
        }
        self.track_tables.push(track_table);
        Ok(())
    }

    pub fn update_track_table(&mut self, track_table: TrackTable) -> Result<(), LiveoError> {
        if let Some(existing) = self
            .track_tables
            .iter_mut()
            .find(|t| t.table_name() == track_table.table_name())
        {
            *existing = track_table;
            Ok(())
        } else {
            return Err(LiveoError::new(
                "Updating track table",
                Some("TrackTable not found".into()),
            ));
        }
    }

    pub fn remove_track_table(&mut self, track_table: &TrackTable) {}

    pub async fn flush(&mut self, client: &mut Client) -> Result<(), Error> {
        for op in &self.operations {
            op.execute(client).await?;
        }

        Ok(())
    }
}

enum Operation {
    Create(TrackTable),
    Update(TrackTable),
    Delete(TrackTable),
}

impl Operation {
    pub async fn execute(&self, client: &mut Client) -> Result<(), Error> {
        match self {
            Operation::Create(track_table) => {
                let transaction = client.transaction().await?;
                let sql_table = queries::drop_tracking_table_sql(&track_table);
                let stmt_table = transaction.prepare(&sql_table).await?;
                transaction.execute(&stmt_table, &[]).await?;
                let sql_func = queries::create_trigger_function_sql(&track_table);
                let stmt_func = transaction.prepare(&sql_func).await?;
                transaction.execute(&stmt_func, &[]).await?;
                let sql_trigger = queries::create_trigger_sql(&track_table);
                let stmt_trigger = transaction.prepare(&sql_trigger).await?;
                transaction.execute(&stmt_trigger, &[]).await?;
                transaction.commit().await?;
                Ok(())
            }
            Operation::Update(track_table) => {
                let transaction = client.transaction().await?;
                let sql_drop = queries::drop_tracking_table_sql(&track_table);
                let stmt_drop = transaction.prepare(&sql_drop).await?;
                transaction.execute(&stmt_drop, &[]).await?;
                let sql_table = queries::drop_tracking_table_sql(&track_table);
                let stmt_table = transaction.prepare(&sql_table).await?;
                transaction.execute(&stmt_table, &[]).await?;
                let sql_func = queries::create_trigger_function_sql(&track_table);
                let stmt_func = transaction.prepare(&sql_func).await?;
                transaction.execute(&stmt_func, &[]).await?;
                let sql_trigger = queries::create_trigger_sql(&track_table);
                let stmt_trigger = transaction.prepare(&sql_trigger).await?;
                transaction.execute(&stmt_trigger, &[]).await?;
                transaction.commit().await?;
                Ok(())
            }
            Operation::Delete(track_table) => {
                let transaction = client.transaction().await?;
                let sql_drop = queries::drop_tracking_table_sql(&track_table);
                let stmt_drop = transaction.prepare(&sql_drop).await?;
                transaction.execute(&stmt_drop, &[]).await?;
                transaction.commit().await?;
                Ok(())
            }
        }
    }
}
