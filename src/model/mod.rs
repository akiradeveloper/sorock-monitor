use super::*;

mod stream;

#[derive(Default)]
pub struct LogState {
    pub head_index: u64,
    pub snapshot_index: u64,
    pub app_index: u64,
    pub commit_index: u64,
    pub last_index: u64,
}

use tokio::task::AbortHandle;
struct DropHandle(AbortHandle);
impl Drop for DropHandle {
    fn drop(&mut self) {
        self.0.abort();
    }
}

#[derive(Default)]
pub struct NodeState {
    pub log_state: LogState,
    drop_log_metrics_stream: Option<DropHandle>,
}

#[derive(Default)]
pub struct Nodes {
    pub nodes: HashMap<Uri, NodeState>,
}
impl Nodes {
    pub async fn update_membership(&mut self, new_membership: HashSet<Uri>) {
        let mut del_list = vec![];
        for (uri, _) in self.nodes.iter() {
            if !new_membership.contains(uri) {
                del_list.push(uri.clone());
            }
        }
        for uri in del_list {
            self.nodes.remove(&uri);
        }
        for uri in new_membership {
            self.nodes.entry(uri.clone()).or_default();
        }
    }
}

pub struct Model {
    pub nodes: Arc<RwLock<Nodes>>,
}
impl Model {
    pub fn connect(addr: Uri, shard_id: u32) -> Self {
        let data = Arc::new(RwLock::new(Nodes::default()));

        tokio::spawn({
            let data = data.clone();
            async move {
                let mut membership = stream::Membership::connect(addr, shard_id).await.unwrap();
                loop {
                    membership.consume(data.clone()).await.ok();
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        });

        tokio::spawn({
            let data = data.clone();
            async move {
                loop {
                    stream::log_metrics::dispatch(data.clone(), shard_id);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        });

        Self { nodes: data }
    }

    pub fn test() -> Self {
        let mut nodes = Nodes::default();
        nodes.nodes.insert(
            Uri::from_static("http://unko:3000"),
            NodeState {
                log_state: model::LogState {
                    head_index: 100,
                    snapshot_index: 110,
                    app_index: 140,
                    commit_index: 160,
                    last_index: 165,
                },
                drop_log_metrics_stream: None,
            },
        );
        nodes.nodes.insert(
            Uri::from_static("http://kuso:3000"),
            NodeState {
                log_state: model::LogState {
                    head_index: 125,
                    snapshot_index: 130,
                    app_index: 140,
                    commit_index: 165,
                    last_index: 180,
                },
                drop_log_metrics_stream: None,
            },
        );
        Self {
            nodes: Arc::new(RwLock::new(nodes)),
        }
    }
}
