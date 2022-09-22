use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct AgentInfo {
    pub agentid: i32,
    pub agentname: String,
    pub connected: i32,
    pub ipaddress: String
}