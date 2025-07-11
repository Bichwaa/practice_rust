mod project;
mod notes;

//use crate::project;

fn main() {
    let prj = create_test_project();
    println!("Hello, world! {prj:?}");
}

fn string(s: &str)->String{
    String::from(s)
}

fn create_test_project()-> project::Project {
    let mut pr = project::Project::new(1, string("testy"), string("destriptive"));
    let mut obj = project::Objective::new(0, string("objective 1"), string("also desctriptive"));
    let mut goal = project::Goal::new(0, string("new goal"), string("is to?"));
    let mut task = project::Task::new(0, string("a task"), string("another task fr"));
    let mut note = notes::Note::new(0, string("a note for everybody"), string("this is the note body"));
    note.title = string("I can change her");
    println!("the note title: {:?}", note.title);
    task.notes.push(note.clone());
    goal.tasks.push(task);
    goal.notes.push(note.clone());
    obj.goals.push(goal);
    obj.notes.push(note.clone());
    pr.objectives.push(obj);
    pr.notes.push(note);
    pr
}
