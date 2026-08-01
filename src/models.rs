#![allow(unused)]
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context
{
    created: DateTime<Utc>,
}

impl Context
{
    pub fn day<T: Into<DateTime<Utc>>>(created: T) -> Context
    {
        Context {
            created: created.into(),
        }
    }
}
