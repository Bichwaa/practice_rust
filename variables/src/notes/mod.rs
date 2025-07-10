#[derive(Debug)]
pub struct Note {
    id: usize,
    title: String,
    body: String
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

