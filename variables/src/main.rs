mod project;
mod notes;

use crate::project::Project;

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

fn create_test_project()-> Project {
    let mut pr = crate::project::Project::new(1, string("testy"), string("destriptive"));
    let mut obj = crate::project::Objective::new(0, string("objective 1"), string("also desctriptive"));
    let goal = crate::project::Goal::new(0, string("new goal"), string("is to?"));
    obj.goals.push(goal);
    pr.objectives.push(obj);
    pr
}
