use super::*;

pub struct Membership {}
impl Membership {
    pub async fn connect(addr: Uri) -> Result<Self> {
        todo!()
    }

    pub async fn consume(&mut self, nodes: Arc<RwLock<Nodes>>) {
        todo!()
    }
}
