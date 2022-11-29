use serde::{Deserialize, Serialize};
use async_graphql::{EmptyMutation, EmptySubscription, Schema, SimpleObject, ComplexObject};
use chrono::{DateTime, Local};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::types::Timestamp;

mod model;
pub use model::QueryRoot;

pub type PerfSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(SimpleObject, PostgresMapper, Debug)]
#[graphql(complex)]
#[pg_mapper(table = "real_time_perf")]
pub struct Realtimeperf {
    agent_id: i32,
    agent_name: String,
    ontune_time: DateTime<Local>,
    _user: i32,
    sys: i32,
    idle: i32,
    processor_count: i32,
    run_queue: i32,
    block_queue: i32,
    wait_queue: i32,
    p_queue: i32,
    p_crate_user: i32,
    p_crate_sys: i32,
    memory_size: i32,
    memory_used: i32,
    memory_pinned: i32,
    memory_sys: i32,
    memory_user: i32,
    memory_cache: i32,
    avm: i32,
    paging_space_in: i32,
    paging_space_out: i32,
    file_system_in: i32,
    file_system_out: i32,
    memory_scan: i32,
    memory_freed: i32,
    swap_size: i32,
    swap_used: i32,
    swap_active: i32,
    fork: i32,
    exec: i32,
    interrupt: i32,
    system_call: i32,
    context_switch: i32,
    semaphore: i32,
    msg: i32,
    disk_read_write: i32,
    disk_iops: i32,
    network_read_write: i32,
    network_iops: i32,
    top_command_id: i32,
    top_command_count: i32,
    top_user_id: i32,
    top_cpu: i32,
    top_disk_id: i32,
    top_vg_id: i32,
    top_busy: i32,
    max_pid: i32,
    thread_count: i32,
    pid_count: i32,
    linux_buffer: i32,
    linux_cached: i32,
    linux_srec: i32,
    mem_used_mb: i32,
    irq: i32,
    soft_irq: i32,
    swap_used_mb: i32,
    dusm: i32
}

#[ComplexObject]
impl Realtimeperf {}