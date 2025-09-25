use std::time::Duration;
use crate::traits::SharedFunctionality;

// === State structs ===
#[derive(Debug)]
pub struct Waiting {
    pub waiting_time: Duration,
    pub shared_value: usize,
}

impl SharedFunctionality for Waiting {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

#[derive(Debug)]
pub struct Filling {
    pub rate: usize,
    pub shared_value: usize,
}

impl SharedFunctionality for Filling {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

#[derive(Debug)]
pub struct Done {
    pub bottles_filled: usize,
    pub shared_value: usize,
}

impl SharedFunctionality for Done {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

#[derive(Debug)]
pub struct Cleaning {
    pub cleaning_cycles: usize,
    pub shared_value: usize,
}

impl SharedFunctionality for Cleaning {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

#[derive(Debug)]
pub struct Maintenance {
    pub maintenance_hours: f64,
    pub shared_value: usize,
}

impl SharedFunctionality for Maintenance {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

#[derive(Debug)]
pub struct Error {
    pub error_code: String,
    pub shared_value: usize,
}

impl SharedFunctionality for Error {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

// === Enum wrapper ===
#[derive(Debug)]
pub enum State {
    Waiting(Waiting),
    Filling(Filling),
    Done(Done),
    Cleaning(Cleaning),
    Maintenance(Maintenance),
    Error(Error),
}