use super::*;

mod nodes;
mod progress_log;
mod stream;
pub use nodes::*;
pub use progress_log::*;

pub struct Model {
    pub nodes: Arc<RwLock<Nodes>>,
    pub progress_log: Arc<RwLock<ProgressLog>>,
}
impl Model {
    pub fn connect(addr: Uri, shard_id: u32) -> Self {
        let nodes = Arc::new(RwLock::new(Nodes::default()));
        let progress_log = Arc::new(RwLock::new(ProgressLog::new()));

        tokio::spawn({
            let nodes = nodes.clone();
            async move {
                let mut membership = stream::Membership::connect(addr, shard_id).await.unwrap();
                loop {
                    membership.consume(nodes.clone()).await.ok();
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        });

        tokio::spawn({
            let nodes = nodes.clone();
            async move {
                loop {
                    nodes::dispatch(nodes.clone(), shard_id);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        });

        tokio::spawn({
            let nodes = nodes.clone();
            let progress_log = progress_log.clone();
            async move {
                loop {
                    progress_log::copy(nodes.clone(), progress_log.clone());
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        });

        Self {
            nodes,
            progress_log,
        }
    }

    pub fn test() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(Nodes::test())),
            progress_log: Arc::new(RwLock::new(ProgressLog::test())),
        }
    }
}
