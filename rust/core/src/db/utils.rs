use std::error::Error;

/**
 * Utility functions for database operations
 */

/// Helper function to validate SQL identifiers
pub fn is_valid_identifier(name: &str, err_out: &'static str) -> Result<(), Box<dyn Error>> {
    if name.is_empty() || name.len() > 63 {
        return Err(err_out.into());
    }

    // Must start with letter or underscore
    let first_char = name.chars().next().unwrap();
    if !first_char.is_ascii_alphabetic() && first_char != '_' {
        return Err(err_out.into());
    }

    // Rest must be alphanumeric or underscore
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(err_out.into());
    }

    Ok(())
}
