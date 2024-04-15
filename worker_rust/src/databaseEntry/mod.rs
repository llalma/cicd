use crate::stage::Stage;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct entry_pipeline {
    pub id: String,
    pub name: String,
    pub stages: Vec<Stage>,
}
