use super::*;

pub struct Membership {
    shard_id: u32,
    conn: proto::monitor_client::MonitorClient<Channel>,
}
impl Membership {
    pub async fn connect(addr: Uri, shard_id: u32) -> Result<Self> {
        let endpoint = Endpoint::new(addr)?;
        let chan = endpoint.connect().await?;
        Ok(Self {
            shard_id,
            conn: proto::monitor_client::MonitorClient::new(chan),
        })
    }
    pub async fn consume(&mut self, nodes: Arc<RwLock<Nodes>>) -> Result<()> {
        let new_membership = {
            let mut out = HashSet::new();
            let mem = self
                .conn
                .get_membership(proto::Shard { id: self.shard_id })
                .await?
                .into_inner();
            for mem in mem.members {
                let url = Uri::from_maybe_shared(mem).unwrap();
                out.insert(url);
            }
            out
        };
        let mut nodes = nodes.write();
        nodes.update_membership(new_membership).await;

        Ok(())
    }
}
