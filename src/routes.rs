use crate::owodomain::OwoDomain;
use parking_lot::RwLock;
use rocket::{get, State};
use rocket_contrib::json::JsonValue;
use std::{collections::HashMap, sync::Arc};

#[get("/")]
pub fn check_domains(
    state: State<Arc<RwLock<HashMap<OwoDomain, bool>>>>,
) -> Result<JsonValue, serde_json::Error> {
    Ok(JsonValue(serde_json::to_value(state.inner())?))
}
