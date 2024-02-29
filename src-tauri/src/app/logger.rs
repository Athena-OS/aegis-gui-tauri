use tracing::Subscriber;
use tracing_core::Event;

use tracing_subscriber::Layer;

use std::fs::File;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

#[derive(Debug)]

pub struct FileLogger {
    file: Arc<Mutex<File>>,
}

impl FileLogger {
    #[allow(dead_code)]
    pub fn new(path: &str) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(FileLogger {
            file: Arc::new(Mutex::new(file)),
        })
    }
    #[allow(dead_code)]
    pub fn write(&self, buf: &[u8]) -> io::Result<()> {
        let mut file = self.file.lock().unwrap();
        file.write_all(buf)?;
        Ok(())
    }

    pub fn writeln(&self, args: std::fmt::Arguments) -> io::Result<()> {
        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", args)
    }
}

pub struct FileLoggerLayer {
    logger: Arc<FileLogger>,
}

impl FileLoggerLayer {
    #[allow(dead_code)]
    pub fn new(logger: Arc<FileLogger>) -> Self {
        FileLoggerLayer { logger }
    }
}

impl<S: Subscriber> Layer<S> for FileLoggerLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let meta = event.metadata();
        if let Ok(_) = self
            .logger
            .writeln(format_args!("{}: {:#?}", meta.target(), event))
        {
            // Handle write error or ignore
        }
    }
}

/*use tracing::subscriber::set_global_default;
use tracing_subscriber::{util::SubscriberInitExt, Registry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_logger = Arc::new(FileLogger::new("app.log")?);
    let file_logger_layer = FileLoggerLayer::new(file_logger);

    let subscriber = tracing_subscriber::Registry::default().with(file_logger_layer);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Your application logic here
    tracing::info!("testing");
    tracing::info!("testing 2");
    println!("testing 3");
    Ok(())
}*/
