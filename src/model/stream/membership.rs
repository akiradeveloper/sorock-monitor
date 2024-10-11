use super::*;

pub struct Membership {
    conn: proto::monitor_client::MonitorClient<Channel>,
}
impl Membership {
    pub async fn connect(addr: Uri) -> Result<Self> {
        let endpoint = Endpoint::new(addr)?;
        let chan = endpoint.connect().await?;
        Ok(Self {
            conn: proto::monitor_client::MonitorClient::new(chan),
        })
    }
    pub async fn consume(&mut self, nodes: Arc<RwLock<Nodes>>) {
        loop {
            let new_membership = {
                let mut out = HashSet::new();
                out
            };
            let mut nodes = nodes.write();
            nodes.update_membership(new_membership).await;
        }
    }
}
