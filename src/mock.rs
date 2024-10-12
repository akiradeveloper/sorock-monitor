mod proto {
    tonic::include_proto!("sorock_monitor");
}
use proto::*;

use futures::stream::Stream;
use std::time::Instant;
use std::{pin::Pin, time::Duration};
use tonic::transport::{Server, Uri};

pub struct App {
    url: Uri,
    start_time: Instant,
}
impl App {
    pub fn new(url: Uri) -> Self {
        Self {
            url,
            start_time: Instant::now(),
        }
    }
}
#[tonic::async_trait]
impl proto::monitor_server::Monitor for App {
    async fn get_membership(
        &self,
        _: tonic::Request<Shard>,
    ) -> std::result::Result<tonic::Response<Membership>, tonic::Status> {
        let out = Membership {
            members: vec![self.url.clone().to_string()],
        };
        Ok(tonic::Response::new(out))
    }

    type GetLogMetricsStream =
        Pin<Box<dyn Stream<Item = Result<LogMetrics, tonic::Status>> + Send>>;

    async fn get_log_metrics(
        &self,
        _: tonic::Request<Shard>,
    ) -> std::result::Result<tonic::Response<Self::GetLogMetricsStream>, tonic::Status> {
        let start_time = self.start_time;
        let st = async_stream::try_stream! {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                let x = Instant::now().duration_since(start_time).as_secs();
                let pow = |x: u64| {
                    x*x
                };
                let metrics = LogMetrics {
                    head_index: pow(x),
                    snap_index: pow(x+1),
                    app_index: pow(x+2),
                    commit_index: pow(x+3),
                    last_index: pow(x+4),
                };
                yield metrics
            }
        };
        Ok(tonic::Response::new(Box::pin(st)))
    }
}

pub fn launch_mock_server() {
    tokio::spawn(async move {
        let addr: Uri = "http://localhost:50051".parse().unwrap();
        let app = App::new(addr);
        let sock = "0.0.0.0:50051".parse().unwrap();
        Server::builder()
            .add_service(proto::monitor_server::MonitorServer::new(app))
            .serve(sock)
            .await
            .unwrap();
    });
    std::thread::sleep(Duration::from_secs(1));
}
