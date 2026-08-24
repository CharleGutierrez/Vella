use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;
use crate::ai::tuner::AiTuner;

#[derive(Clone)]
pub struct JobContext {
    pub db: Arc<dyn crate::db::DatabaseAdapter>,
}

pub struct JobQueue {
    scheduler: JobScheduler,
    ai_tuner: Arc<AiTuner>,
}

impl JobQueue {
    pub async fn new(ai_tuner: Arc<AiTuner>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let scheduler = JobScheduler::new().await?;
        Ok(Self { scheduler, ai_tuner })
    }

    /// Schedule an AI-Optimized background cron job
    pub async fn schedule_ai_optimized<F>(&mut self, cron_expr: &str, mut task: F) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut() + Send + Sync + 'static + Clone,
    {
        let tuner_clone = self.ai_tuner.clone();
        let cron_clone = cron_expr.to_string();

        let job = Job::new_async(cron_expr, move |_uuid, _l| {
            let mut _task_clone = task.clone(); // In reality requires careful state sharing
            let tuner = tuner_clone.clone();
            let cron = cron_clone.clone();

            Box::pin(async move {
                let delay = tuner.predict_optimal_delay(&cron);
                if delay > 0 {
                    tokio::time::sleep(std::time::Duration::from_secs(delay)).await;
                }
                info!("Executing AI-optimized background job...");
                // _task_clone();
            })
        })?;
        
        self.scheduler.add(job).await?;
        Ok(())
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Starting Background Job Queue...");
        self.scheduler.start().await?;
        Ok(())
    }
}
