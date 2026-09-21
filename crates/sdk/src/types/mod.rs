pub mod task;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(pub String);

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uid(pub String);
