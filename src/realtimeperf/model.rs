use async_graphql::{Context, FieldResult, Object};
use deadpool_postgres::{Client, Pool};
use super::{Realtimeperf};
use crate::errors::MyError;
use tokio_pg_mapper::FromTokioPostgresRow;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn realtimeperf(&self, ctx: &Context<'_>, id: i32) -> FieldResult<Vec<Realtimeperf>> {

        let pool = ctx.data::<Pool>().unwrap();
        let client: Client = pool.get().await.map_err(MyError::PoolError)?;

        let query_str = format!("SELECT agent_id, agent_name, ontune_time, _user, sys, idle, processor_count, run_queue, block_queue, wait_queue, p_queue, p_crate_user, p_crate_sys, memory_size, memory_used, memory_pinned, memory_sys, memory_user, memory_cache, avm, paging_space_in, paging_space_out, file_system_in, file_system_out, memory_scan, memory_freed, swap_size, swap_used, swap_active, fork, exec, interrupt, system_call, context_switch, semaphore, msg, disk_read_write, disk_iops, network_read_write, network_iops, top_command_id, top_command_count, top_user_id, top_cpu, top_disk_id, top_vg_id, top_busy, max_pid, thread_count, pid_count, linux_buffer, linux_cached, linux_srec, mem_used_mb, irq, soft_irq, swap_used_mb, dusm \
                                        FROM real_time_perf \
                                        WHERE agent_id = $1");
        let stmt = client.prepare(&query_str).await.unwrap();

        let result = client
            .query(&stmt, &[&id])
            .await?
            .iter()
            .map(|row| Realtimeperf::from_row_ref(row).unwrap())
            .collect::<Vec<Realtimeperf>>()
            ;

        return Ok(result);
    }
}