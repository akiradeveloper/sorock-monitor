use super::*;

pub struct LogMetrics {
    conn: proto::monitor_client::MonitorClient<Channel>,
}
impl LogMetrics {
    pub fn connect(url: Uri, shard_id: u32) -> Self {
        todo!()
    }

    pub async fn consume(&mut self) -> Result<()> {
        let mut st = self.conn.get_log_metrics(()).await?.into_inner();
        while let Some(metric) = st.message().await? {
            println!("{:?}", metric);
        }
        Ok(())
    }
}
