use http::Uri;

use super::*;

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
    log_state: LogState,
}

#[derive(Default)]
pub struct Nodes {
    nodes: HashMap<Uri, NodeState>,
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
