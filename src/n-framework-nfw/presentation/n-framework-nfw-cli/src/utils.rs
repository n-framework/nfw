use uuid::Uuid;

/// Generates a short, unique 8-character ID for error logging and tracking.
pub fn generate_error_id() -> String {
    Uuid::new_v4().as_simple().to_string()[..8].to_string()
}
