pub mod common;
extern crate postgres;
use postgres::{Client, NoTls};
use crate::agent::common::AgentInfo;

static PG_CONNECTION_INFO: &str = "postgres://aa:aa@127.0.0.1:5432/aa";

pub fn get_agents(vec: &mut Vec<common::AgentInfo>) {

    let mut client = Client::connect(PG_CONNECTION_INFO, NoTls).unwrap();

    let query = "SELECT _agentid, _agentname, _connected, _ipaddress FROM agentinfo WHERE _enabled = 1";
    println!("{}",query);

    for row in client.query(query, &[]).unwrap() {
        let result = common::AgentInfo {
            agentid: row.get(0),
            agentname: row.get(1),
            connected: row.get(2),
            ipaddress: row.get(3)
        };
        vec.push(result);
    }
}