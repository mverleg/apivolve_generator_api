
use ::serde::Deserialize;
use ::serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateSteps {

}

impl GenerateSteps {
    pub fn new() -> Self {
        GenerateSteps {}
    }
}
