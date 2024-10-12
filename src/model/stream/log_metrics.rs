use super::*;

pub struct LogMetrics {
    url: Uri,
    shard_id: u32,
    conn: proto::monitor_client::MonitorClient<Channel>,
}
impl LogMetrics {
    pub fn connect(url: Uri, shard_id: u32) -> Self {
        let endpoint = Endpoint::new(url.clone()).unwrap();
        let chan = endpoint.connect_lazy();
        Self {
            url,
            shard_id,
            conn: proto::monitor_client::MonitorClient::new(chan),
        }
    }
    pub async fn consume(&mut self, data: Arc<RwLock<Nodes>>) -> Result<()> {
        let mut st = self
            .conn
            .get_log_metrics(proto::Shard { id: self.shard_id })
            .await?
            .into_inner();
        while let Some(metric) = st.message().await? {
            if let Some(state) = data.write().nodes.get_mut(&self.url) {
                let new_state = LogState {
                    head_index: metric.head_index,
                    snapshot_index: metric.snap_index,
                    app_index: metric.app_index,
                    commit_index: metric.commit_index,
                    last_index: metric.last_index,
                };
                state.log_state = new_state;
            }
        }
        Ok(())
    }
}
