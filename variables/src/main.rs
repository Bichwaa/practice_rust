mod project;

fn main() {
    let prj = crate::project::Project::new(0, String::from("neo"), String::from("the very first one"));
    let mut x = String::from("na wengine");
    println!("Hello, world! {x}");
    x.push('o');
    println!("Hello, world! {prj:?}");
}
