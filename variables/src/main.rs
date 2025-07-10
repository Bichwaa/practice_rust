mod project;
mod notes;

//use crate::project;

fn main() {
    let prj = create_test_project();
    let mut x = String::from("na wengine");
    println!("Hello, world! {x}");
    x.push('o');
    println!("Hello, world! {prj:?}");
}

fn string(s: &str)->String{
    String::from(s)
}

fn create_test_project()-> project::Project {
    let mut pr = project::Project::new(1, string("testy"), string("destriptive"));
    let mut obj = project::Objective::new(0, string("objective 1"), string("also desctriptive"));
    let mut goal = project::Goal::new(0, string("new goal"), string("is to?"));
    let task = project::Task::new(0, string("a task"), string("another task fr"));
    goal.tasks.push(task);
    obj.goals.push(goal);
    pr.objectives.push(obj);
    pr
}
