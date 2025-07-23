use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    Todo,
    Complete,
    Underway,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct Progress(f64);

impl Progress {
    pub fn new(value: f64) -> Result<Self, String> {
        if value >= 0.0 && value <= 1.0 {
            Ok(Progress(value))
        } else {
            Err(format!(
                "Invalid value must be between 0.0 and 1.0. Recieved: {}",
                value
            ))
        }
    }

    pub fn one() -> Self {
        Progress(1.0)
    }

    pub fn zero() -> Self {
        Progress(0.0)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0.0
    }

    pub fn is_one(&self) -> bool {
        self.0 == 1.0
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    status: Status,
    desc: String,
    deadline: Option<NaiveDate>,
    progress: Progress,
}

impl Item {
    pub fn new(desc: String) -> Self {
        Item {
            status: Status::Todo,
            desc,
            deadline: None,
            progress: Progress::zero(),
        }
    }

    pub fn due(mut self, deadline: NaiveDate) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub fn set_deadline(&mut self, deadline: NaiveDate) {
        self.deadline = Some(deadline);
    }

    pub fn set_progress(&mut self, progress: Progress) {
        let status = match &progress {
            p if p.is_one() => Status::Complete,
            p if p.is_zero() => Status::Todo,
            _ => Status::Underway,
        };
        self.status = status;
        self.progress = progress;
    }

    pub fn set_desc(&mut self, desc: String) {
        self.desc = desc;
    }

    pub fn deadline(&self) -> &Option<NaiveDate> {
        &self.deadline
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn progress(&self) -> &Progress {
        &self.progress
    }

    pub fn desc(&self) -> &String {
        &self.desc
    }
}
