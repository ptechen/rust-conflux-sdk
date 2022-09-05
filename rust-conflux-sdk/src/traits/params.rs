use serde::{Serialize};
use iter_tools::iter::Itertools;

#[derive(Serialize, Debug)]
pub struct ConfluxParams {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Vec<String>>,
}

impl ConfluxParams {
    pub fn new(id: u64, method: &str, params: Option<Vec<&str>>) -> Self {
        if params.is_none() {
            Self {
                id: id.to_string(),
                jsonrpc: "2.0".to_owned(),
                method: method.to_owned(),
                params: None,
            }
        } else {
            let params: Vec<String> = params.unwrap().iter().map(|a| a.to_string()).collect_vec();
            Self {
                id: id.to_string(),
                jsonrpc: "2.0".to_owned(),
                method: method.to_owned(),
                params: Some(params),
            }
        }
    }
}
