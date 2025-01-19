use rocket::fairing::{Fairing, Info};
use rocket::{Data, Orbit, Request, Response, Rocket};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[allow(unused)]
#[derive(Clone)]
pub(crate) struct Logger {
    file: std::path::PathBuf,
    log_level: u8,
}

#[allow(unused)]
impl Logger {
    pub(crate) fn new(file_path: &str, log_level: u8) -> Logger {
        let file = std::path::PathBuf::from(file_path);

        Logger { file, log_level }
    }

    pub(crate) fn log(&mut self, msg: &str) {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.clone().file);

        match file {
            Ok(mut file) => file.write_all(msg.as_bytes()).unwrap_or(()),
            Err(_) => {
            }
        }
    }
}

#[allow(unused)]
pub(crate) struct FairingLogger {
    logger: Arc<Mutex<Logger>>,
}

impl FairingLogger {
    pub(crate) fn new(logger: Arc<Mutex<Logger>>) -> FairingLogger {
        FairingLogger { logger }
    }

    pub(crate) fn logger(&self) -> Arc<Mutex<Logger>> {
        Arc::clone(&self.logger)
    }
}

// Code for the chrono of the request handling is from the Rocket documentation : https://api.rocket.rs/v0.5/rocket/fairing/trait.Fairing#example
#[derive(Copy, Clone)]
struct TimerStart(Option<SystemTime>);

#[allow(unused)]
#[rocket::async_trait]
impl Fairing for FairingLogger {
    fn info(&self) -> Info {
        Info {
            name: "Logger",
            kind: rocket::fairing::Kind::Request
                | rocket::fairing::Kind::Response
                | rocket::fairing::Kind::Liftoff
                | rocket::fairing::Kind::Shutdown,
        }
    }

    async fn on_liftoff(&self, _rocket: &Rocket<Orbit>) {
        self.logger()
            .lock()
            .unwrap()
            .log(&format!("{} - Server started successfully\n", get_time()));
        // Add launched app state and config info
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        req.local_cache(|| TimerStart(Some(SystemTime::now())));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        // Message to log, with the time, the method, the IP, and the URI
        let mut msg = format!(
            "{} - {} Request received from user {} ip: {} to uri \"{}\"",
            get_time(),
            req.method(),
            req.headers().get_one("X-USER-NAME").unwrap_or("No user name"),
            get_ip(req),
            req.uri()
        );

        // Check if the API key is valid, write the message accordingly
        match res.status().code {
            401 => {
                msg = format!(
                    "{msg} - Invalid API key : {}",
                    req.headers()
                        .get_one("X-API-KEY")
                        .unwrap_or("No API key provided")
                );
            }
            _ => {
                msg = format!("{msg} - Valid API key");
            }
        }

        // Status code
        let msg = format!("{msg} - Status {}", res.status().code);

        // Response time
        let start_time = req.local_cache(|| TimerStart(None));
        if let Some(Ok(duration)) = start_time.0.map(|st| st.elapsed()) {
            let ms = (duration.as_secs() * 1000) as f64 + duration.subsec_micros() as f64 / 1000f64;
            res.set_raw_header("X-Response-Time", format!("{} ms", ms));
        }
        let msg = format!(
            "{msg} - Response time : {}\n",
            res.headers()
                .get_one("X-Response-Time")
                .unwrap_or("Unknown")
        );

        // Log the message
        self.logger().lock().unwrap().log(&msg);
    }

    async fn on_shutdown(&self, _rocket: &Rocket<Orbit>) {
        self.logger()
            .lock()
            .unwrap()
            .log(&format!("{} - Server shutting down\n", get_time()));
    }
}

fn get_time() -> String {
    let time = chrono::Local::now();
    time.format("%Y/%m/%d %H:%M:%S").to_string()
}

fn get_ip(req: &Request) -> String {
    let ip = req.client_ip();
    match ip {
        Some(ip) => ip.to_string(),
        None => String::from("Unknown IP"),
    }
}
