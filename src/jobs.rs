use std::sync::{Arc, Mutex};
use std::time::Duration;
use chrono::Utc;
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::logs::Logger;

pub(crate) async fn remove_backup_job(data_dir: String, logger: Arc<Mutex<Logger>>) {
    // Start the scheduler
    let scheduler = JobScheduler::new().await.unwrap();

    let job = Job::new_tz("0 0 2 * * *", Utc, move |_, _| {
        // Runs at 2 AM every day
        let data_dir = data_dir.clone();
        let logger = Arc::clone(&logger);

        tokio::spawn(async move {
            logger.lock().unwrap().log("Running remove_backup_job");

            // If read data_dir is Ok, iterate over each
            if let Ok(user_directories) = std::fs::read_dir(data_dir) {
                user_directories.for_each(|dir| {
                    // If read user_dir is Ok, push backup to the path and iterate over each file
                    let mut backup_dir = dir.unwrap().path();
                    backup_dir.push("backup");

                    if let Ok(files) = std::fs::read_dir(backup_dir) {
                        files.for_each(|file| {
                            let file = file.unwrap();
                            let metadata = file.metadata().unwrap();
                            let duration = metadata.modified().unwrap().elapsed().unwrap();
                            if duration.as_secs() > 604800 { // Seven days
                                std::fs::remove_file(file.path()).unwrap();
                                logger.lock().unwrap().log(&format!("Removed {:?} file", file.path()));
                            }
                        });
                    }
                });
            }
        });
    })
        .unwrap();

    scheduler.add(job).await.unwrap();

    // Start the scheduler
    scheduler.start().await.unwrap();

    // Keep the thread alive
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
        // println!("Alive");
    }
}