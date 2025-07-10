use crate::notes::{Note};

#[derive(Debug)]
pub struct Task {
    id: usize,
    name: String,
    description: String,
    pub notes: Vec<Note>
}

impl  Task {
    pub fn new(id:usize, name: String, description: String)-> Task{
        Task{
            id,
            name,
            description,
            notes:Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Goal {
    id: usize,
    name: String,
    description: String,
    pub tasks: Vec<Task>,
    pub notes: Vec<Note>
}

impl Goal {
    pub fn new(id: usize, name: String, description: String)->Goal {
    Goal{
    id,
    name,
    description,
    tasks: Vec::new(),
    notes: Vec::new()
}
}
}

#[derive(Debug)]
pub struct Objective {
    id: usize,
    name: String,
    description: String,
    pub goals:Vec<Goal>,
    pub notes: Vec<Note>
}

impl Objective {
    pub fn new(id:usize, name: String, description: String) -> Objective{
        Objective{
            id,
            name,
            description,
            goals: Vec::new(),
            notes:Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Project {
    id: usize,
    name: String,
    description: String,
    pub objectives: Vec<Objective>,
    pub notes: Vec<Note>
}

impl Project {
    pub fn new(id:usize, name: String, description: String) -> Project{
        Project {
            id,
            name,
            description,
            objectives: Vec::new(),
            notes: Vec::new()
        }
    }
}
