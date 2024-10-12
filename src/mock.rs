mod proto {
    tonic::include_proto!("sorock_monitor");
}
use proto::*;

use futures::stream::Stream;
use std::pin::Pin;
use std::time::Instant;
use tonic::transport::Uri;

struct App {
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
        _: tonic::Request<()>,
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
        _: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<Self::GetLogMetricsStream>, tonic::Status> {
        let start_time = self.start_time;
        let st = async_stream::try_stream! {
            loop {
                let x = Instant::now().duration_since(start_time).as_secs();
                let pow = |x: u64| {x*x};
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
