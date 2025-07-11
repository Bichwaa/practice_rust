#[derive(Debug, Clone)]
pub struct Note {
    id: usize,
    pub title: String,
    pub body: String
}


impl Note {
    pub fn new (id:usize, title: String, body: String) -> Note {
        Note{
            id,
            title,
            body
        }
    }
}

