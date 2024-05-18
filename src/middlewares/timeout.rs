use std::time::Duration;

use salvo::http::{Request, Response, StatusError};
use salvo::{async_trait, Depot, FlowCtrl, Handler};

/// Timeout with a configurable duration.
pub struct Timeout {
    duration: Duration,
}

impl Timeout {
    pub fn new(duration: Duration) -> Self {
        Timeout { duration }
    }
}

impl Default for Timeout {
    fn default() -> Self {
        Timeout {
            duration: Duration::from_secs(60 * 200), // 200 minutes
        }
    }
}

#[async_trait]
impl Handler for Timeout {
    #[inline]
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        tokio::select! {
            _ = ctrl.call_next(req, depot, res) => {},
            _ = tokio::time::sleep(self.duration) => {
                res.render(StatusError::internal_server_error().brief("Server process the request timeout."));
                ctrl.skip_rest();
            }
        }
    }
}
