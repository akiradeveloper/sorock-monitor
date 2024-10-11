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

#[derive(Default)]
pub struct NodeState {
    pub log_state: LogState,
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
            self.nodes.entry(uri).or_default();
        }
    }
}

pub struct Model {
    pub nodes: Arc<RwLock<Nodes>>,
}
impl Model {
    pub fn new() -> Self {
        let nodes = Arc::new(RwLock::new(Nodes::default()));
        // membership変更のストリーム初期化
        Self { nodes }
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
            },
        );
        Self {
            nodes: Arc::new(RwLock::new(nodes)),
        }
    }
}
