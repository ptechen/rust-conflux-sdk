use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CfxParams<T> {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Vec<T>>,
}

impl<T> CfxParams<T> {
    pub fn new(id: u64, method: &str, params: Option<Vec<T>>) -> Self {
        Self {
            id: id.to_string(),
            jsonrpc: "2.0".to_owned(),
            method: method.to_owned(),
            params,
        }
    }
}
