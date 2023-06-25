#[derive(PartialEq)]
pub struct Process {
    pub name: String,
    pub id: u32,
    pub arrival_time: u32,
    pub execution_time: u32,
    pub start: u32,
    pub end: u32,
    pub waiting_time: u32,
    pub process_time: u32,
    pub reaction_time: u32,
    pub execution_time_left: u32,
    pub started: bool,
    pub completed: bool,
}

impl Process {
    pub fn start(&mut self, time: &u32) {
        self.start = *time;
        println!("At the time point {} the process {} starts.", *time, self.name);
        self.started = true;
        self.execution_time_left = self.execution_time;
        self.reaction_time = self.start - self.arrival_time;
    }

    pub fn complete(&mut self, time: &u32) {
        self.completed = true;
        self.end = *time;
        println!("The process {} ended at the time point {}.", self.name, self.end);
        self.process_time = self.end + 1 - self.arrival_time;
        //debug println!("{}: execution_time: {}, process_time: {}", self.name, self.execution_time, self.process_time);
        self.waiting_time = self.process_time - self.execution_time;
    }

    pub fn process(&mut self, time: &u32) -> bool {
        if self.completed || *time < self.arrival_time {
            false
        } else {
            if !self.started {
                self.start(time);
            }
            println!("At the time point {} the process {} gets processed.", *time, self.name);
            self.execution_time_left -= 1;
            if self.execution_time_left == 0 {
                self.complete(time);
            }
            true
        }
    }
}


impl Clone for Process {
    fn clone(&self) -> Self {
        Process {
            name: self.name.clone(),
            id: self.id,
            arrival_time: self.arrival_time,
            execution_time: self.execution_time,
            start: self.start,
            end: self.end,
            waiting_time: self.waiting_time,
            process_time: self.process_time,
            reaction_time: self.reaction_time,
            execution_time_left: self.execution_time_left,
            started: self.started,
            completed: self.completed,
        }
    }
}
