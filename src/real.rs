use super::*;

use std::{pin::Pin, time::Duration};
use tonic::transport::{Server, Uri};

pub fn connect_real_node(uri: Uri, shard_id: u32) -> impl model::stream::Node {
    let endpoint = Endpoint::from(uri).concurrency_limit(256);
    let channel = endpoint.connect_lazy();
    let client = proto::monitor_client::MonitorClient::new(channel);
    RemoteNode { client, shard_id }
}

struct RemoteNode {
    client: proto::monitor_client::MonitorClient<Channel>,
    shard_id: u32,
}

impl model::stream::Node for RemoteNode {
    fn watch_membership(
        &self,
    ) -> Pin<Box<dyn Stream<Item = proto::Membership> + Send>> {
        let shard = proto::Shard { id: self.shard_id };
        self.client.get_membership(shard).await;
    }

    fn watch_log_metrics(
        &self,
        _: Uri
    ) -> Pin<Box<dyn Stream<Item = proto::LogMetrics> + Send>> {
        let shard = proto::Shard { id: self.shard_id };
        self.client.get_log_metrics(shard).await;
    }
}