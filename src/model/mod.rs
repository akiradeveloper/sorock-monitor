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
                    nodes::dispatch(data.clone(), shard_id);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        });

        Self {
            nodes: data,
            progress_log: Arc::new(RwLock::new(ProgressLog::new())),
        }
    }

    pub fn test() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(Nodes::test())),
            progress_log: Arc::new(RwLock::new(ProgressLog::test())),
        }
    }
}
