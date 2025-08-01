/*
This module is used for managing the workspace,
ie managing the track tables
*/
use super::tracktable::TrackTable;
use tokio_postgres::Client;

pub struct Workspace {
    track_tables: Vec<TrackTable>,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            track_tables: Vec::new(),
        }
    }

    pub fn from_db(client: &Client) -> Result<Self, String> {
        // Placeholder for loading track tables from the database
        // This would typically involve querying the database for existing track tables
        Ok(Workspace::new())
    }
}
