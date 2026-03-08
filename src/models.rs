use chrono::{DateTime, Local, UTC};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    created: DateTime<UTC>,
}

impl Context {
    pub fn day<T: Into<DateTime<UTC>>>(created: T) -> Context {
        Context {
            created: created.into(),
        }
    }
}
