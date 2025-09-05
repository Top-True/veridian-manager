use crate::{application, recorder};
use sqlite::ConnectionThreadSafe;
use uuid::Uuid;

pub struct Database {
    connection: ConnectionThreadSafe,
}

impl Database {
    pub fn new(connection: ConnectionThreadSafe) -> Self {
        let create_table_sql = "CREATE TABLE IF NOT EXISTS application( \
        id TEXT PRIMARY KEY, \
        recorder BLOB NOT NULL \
        )";
        connection.execute(create_table_sql).unwrap();
        Self { connection }
    }
}

impl Database {
    pub fn add_application(&self, application: application::Application) {
        let id = application.id().to_string();
        let recorder_binary = application.recorder().to_binary();
        let mut statement = self
            .connection
            .prepare("INSERT INTO application (id, recorder) VALUES (?, ?)")
            .unwrap();
        statement.bind((1, &*id)).unwrap();
        statement.bind((2, &recorder_binary[..])).unwrap();
        statement.next().unwrap();
    }

    pub fn remove_application(&self, application_id: Uuid) -> Option<application::Application> {
        let application = self.get_application(application_id)?;
        let mut statement = self
            .connection
            .prepare("DELETE FROM application WHERE id = ?")
            .unwrap();
        statement.bind((1, &*application_id.to_string())).unwrap();
        statement.next().unwrap();
        Some(application)
    }

    pub fn get_application(&self, application_id: Uuid) -> Option<application::Application> {
        let mut statement = self
            .connection
            .prepare("SELECT recorder FROM application WHERE id = ?")
            .unwrap();
        statement.bind((1, &*application_id.to_string())).unwrap();
        if let Ok(sqlite::State::Row) = statement.next() {
            let recorder_data = statement.read::<Vec<u8>, &str>("recorder").unwrap();
            let recorder = recorder::Recorder::from_binary(&recorder_data);
            Some(application::Application::new(application_id, recorder))
        } else {
            None
        }
    }
}
