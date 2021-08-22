use serde::Deserialize;

use crate::tasks::Tasks;

#[derive(Deserialize, Debug)]
pub struct JsonFile {
    pub tasks: Tasks,
}
