#[derive(Debug)]
pub struct Task {
    id: usize,
    name: String,
    description: String,
    notes: Vec<String>
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
    tasks: Vec<String>
}

#[derive(Debug)]
pub struct Objective {
    id: usize,
    name: String,
    description: String
    ,
    goals:Vec<Goal>
}

#[derive(Debug)]
pub struct Project {
    id: usize,
    name: String,
    description: String,
    objectives: Vec<Objective>
}

impl Project {
    pub fn new(id:usize, name: String, description: String) -> Project{
        Project {
            id,
            name,
            description,
            objectives: Vec::new()
        }
    }
}
