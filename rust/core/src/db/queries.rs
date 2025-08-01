use crate::db::tracktable::TrackTable;

use super::tracktable::On;

/**
 * SQL queries for database operations
 */

/// Creates a tracking table with dynamic fields based on the columns being tracked
pub fn create_tracking_table_sql(track_table: &TrackTable) -> String {
    // Build dynamic columns based on tracked columns
    let mut columns = vec!["id UUID PRIMARY KEY DEFAULT gen_random_uuid()".to_string()];

    // Add a timestamp column
    columns.push("timestamp TIMESTAMP DEFAULT NOW()".to_string());
    // add the p_id column if it exists
    if let Some(ref col) = track_table.p_id {
        columns.push(format!("p_id {} DEFAULT NULL", col.data_type));
    }
    // Add columns for each tracked field (old and new values)
    for col in &track_table.track {
        if track_table.old {
            columns.push(format!("old_{} {}", col.name, col.data_type))
        }
        if track_table.new {
            columns.push(format!("new_{} {}", col.name, col.data_type))
        }
    }

    let columns_sql = columns.join(", ");

    format!(
        r#"CREATE TABLE IF NOT EXISTS "{}" ({})"#,
        track_table.table_name(), // Escape quotes
        columns_sql
    )
}

/// Creates a trigger function that captures changes and stores them in the tracking table
pub fn create_trigger_function_sql(track_table: &TrackTable) -> String {
    // Build column lists and values based on operation type
    let mut columns = vec!["timestamp".to_string()];
    let mut values = vec!["NOW()".to_string()];

    if let Some(ref p_id_type) = track_table.p_id {
        columns.push("p_id".to_string());
        values.push(format!("NEW.{}", p_id_type.name));
    }

    // Add columns for tracked fields based on old/new flags
    for col in &track_table.track {
        if track_table.old {
            columns.push(format!("old_{}", col.name));
            values.push(format!("OLD.{}", col.name));
        }
        if track_table.new {
            columns.push(format!("new_{}", col.name));
            values.push(format!("NEW.{}", col.name));
        }
    }

    let columns_sql = columns.join(", ");
    let values_sql = values.join(", ");

    // Determine return value based on operation
    let return_statement = match track_table.on {
        On::Delete => "RETURN OLD;",
        _ => "RETURN NEW;",
    };

    format!(
        r#"CREATE OR REPLACE FUNCTION "{}"()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO "{}" ({})
    VALUES ({});
    
    {};
END;
$$ LANGUAGE plpgsql;"#,
        track_table.func_name(),  // Escape quotes in function name
        track_table.table_name(), // Escape quotes in table name
        columns_sql,
        values_sql,
        return_statement
    )
}

/// Creates the actual trigger that calls the trigger function
pub fn create_trigger_sql(track_table: &TrackTable) -> String {
    let timing = match track_table.on {
        On::Insert => "AFTER INSERT",
        On::Update => "AFTER UPDATE",
        On::Delete => "AFTER DELETE",
    };

    // Drop the trigger if it exists, then create the new one
    format!(
        r#"DROP TRIGGER IF EXISTS "{}" ON "{}";
CREATE TRIGGER "{}" 
    {} ON "{}"
    FOR EACH ROW 
    EXECUTE FUNCTION "{}"();"#,
        track_table.trigger_name(),
        track_table.table_name(),
        track_table.trigger_name(),
        timing,
        track_table.table_name(),
        track_table.func_name()
    )
}

pub fn drop_tracking_table_sql(track_table: &TrackTable) -> String {
    let mut batch = Vec::new();
    // drop the tracking table if it exists
    batch.push(format!(
        r#"DROP TABLE IF EXISTS "{}";"#,
        track_table.table_name()
    ));
    // drop the trigger function if it exists
    batch.push(format!(
        r#"DROP FUNCTION IF EXISTS "{}";"#,
        track_table.func_name()
    ));
    // drop the trigger if it exists
    batch.push(format!(
        r#"DROP TRIGGER IF EXISTS "{}" ON "{}";"#,
        track_table.trigger_name(),
        track_table.table_name()
    ));
    batch.join("\n")
}

#[inline]
pub fn create_workspace_table_sql() -> String {
    // Create a workspace table to manage track tables
    r#"CREATE TABLE IF NOT EXISTS liveo_workspace (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        p_id liveo_col,
        table_name TEXT NOT NULL,
        track_op TEXT NOT NULL,
        track_col TEXT[] NOT NULL,
        created_at TIMESTAMP DEFAULT NOW(),
        updated_at TIMESTAMP DEFAULT NOW()
    );"#
    .to_string()
}
#[inline]
pub fn fetch_track_tables_sql() -> String {
    // Fetch existing track tables from the workspace
    r#"SELECT * FROM liveo_workspace;"#.to_string()
}

#[inline]
pub fn create_col_type() -> String {
    r#"CREATE TYPE liveo_col AS (
        name TEXT NOT NULL,
        data_type TEXT NOT NULL
    );"#
    .to_string()
}
