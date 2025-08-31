use crate::recorder;
use uuid::Uuid;

pub struct Application {
    id: Uuid,
    recorder: recorder::Recorder,
}

impl Application {
    pub async fn from_database(id: Uuid) -> Self {
        todo!()
    }
}

impl From<recorder::Recorder> for Application {
    fn from(recorder: recorder::Recorder) -> Self {
        let id = Uuid::new_v4();
        Application { id, recorder }
    }
}
