use crate::recorder;
use uuid::Uuid;

pub struct Application {
    id: Uuid,
    recorder: recorder::Recorder,
}

impl Application {
    pub fn new(id: Uuid, recorder: recorder::Recorder) -> Self {
        Self { id, recorder }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn recorder(&self) -> &recorder::Recorder {
        &self.recorder
    }
}

impl From<recorder::Recorder> for Application {
    fn from(recorder: recorder::Recorder) -> Self {
        let id = Uuid::new_v4();
        Self { id, recorder }
    }
}
