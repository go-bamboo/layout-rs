use std::sync::Arc;

use tokio::sync::Mutex;

#[derive(Debug)]
pub struct IDGen {
    sf: snowflake::SnowflakeIdGenerator,
}

impl IDGen {
    pub async fn new() -> Arc<Mutex<IDGen>> {
        Arc::new(Mutex::new(IDGen {
            sf: snowflake::SnowflakeIdGenerator::new(1, 1),
        }))
    }

    pub async fn id(&mut self) -> i64 {
        self.sf.generate()
    }
}
