use super::*;

pub struct LogMetrics {
    url: Uri,
    shard_id: u32,
    conn: proto::monitor_client::MonitorClient<Channel>,
}
impl LogMetrics {
    pub fn connect(url: Uri, shard_id: u32) -> Self {
        todo!()
    }
    pub async fn consume(&mut self, data: Arc<RwLock<Nodes>>) -> Result<()> {
        let mut st = self.conn.get_log_metrics(()).await?.into_inner();
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

pub fn dispatch(data: Arc<RwLock<Nodes>>, shard_id: u32) {
    let mut nodes = data.write();
    for (uri, state) in &mut nodes.nodes {
        if state.drop_log_metrics_stream.is_none() {
            let hdl = tokio::spawn({
                let uri = uri.clone();
                let data = data.clone();
                async move {
                    let mut stream = stream::LogMetrics::connect(uri, shard_id);
                    stream.consume(data).await.unwrap();
                }
            })
            .abort_handle();
            state.drop_log_metrics_stream = Some(DropHandle(hdl));
        }
    }
}
