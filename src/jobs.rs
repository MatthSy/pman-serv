use std::time::Duration;
use chrono::Utc;
use tokio_cron_scheduler::{Job, JobScheduler};

pub(crate) async fn remove_backup_job(data_dir: String) {

    // Start the scheduler
    let scheduler = JobScheduler::new().await.unwrap();

    // TODO : add logging for deletion of a password and running of the job
    let job = Job::new_tz("0 0 2 * * *", Utc, move |_, _| {
        // Runs at 2 o'clock every day
        let data_dir = data_dir.clone();
        tokio::spawn(async move {

            // If read data_dir is Ok, iterate over each
            if let Ok(user_directories) = std::fs::read_dir(data_dir) {
                user_directories.for_each(|dir| {
                    // If read backup_dir is Ok, iterate over each
                    let mut backup_dir = dir.unwrap().path();
                    backup_dir.push("backup");

                    if let Ok(files) = std::fs::read_dir(backup_dir) {
                        files.for_each(|file| {
                            let file = file.unwrap();
                            let metadata = file.metadata().unwrap();
                            let duration = metadata.modified().unwrap().elapsed().unwrap();
                            if duration.as_secs() > 1 {
                                // Seven days : 604800 secs
                                std::fs::remove_file(file.path()).unwrap();
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

    // Keep the main thread alive
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await; // Sleep to keep the main thread alive
        // println!("Alive");
    }
}