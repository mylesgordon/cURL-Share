use crate::routes::user::types::UserError;
use actix_session::Session;

pub async fn get_user_id(session: &Session) -> Result<i64, UserError> {
    let maybe_user_id = session.get::<i64>("user_id")?;
    let id = maybe_user_id.ok_or_else(|| {
        UserError::SessionGetError("User ID not found within session".to_string())
    })?;
    Ok(id)
}
