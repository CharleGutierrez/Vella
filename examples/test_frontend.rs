use vella::model::schema::ModelSchema;
use vella::model::field::Field;
use vella::ui::react_sdk::generate_react_sdk;
use vella::ui::vue_sdk::generate_vue_sdk;
use vella::ui::angular_sdk::generate_angular_sdk;

fn main() {
    let mut post_schema = ModelSchema::new("Post")
        .description("A blog post")
        .field(Field::string("title").required())
        .field(Field::integer("age"))
        .field(Field::boolean("is_published"));

    let schemas = vec![post_schema];

    println!("--- REACT SDK GENERATED ---");
    let react_code = generate_react_sdk("http://localhost:3000", &schemas);
    // Print just the first few lines that contain the generated interface
    for line in react_code.lines().take(25) {
        println!("{}", line);
    }

    println!("\n--- VUE SDK GENERATED ---");
    let vue_code = generate_vue_sdk("http://localhost:3000", &schemas);
    for line in vue_code.lines().take(25) {
        println!("{}", line);
    }

    println!("\n--- ANGULAR SDK GENERATED ---");
    let angular_code = generate_angular_sdk("http://localhost:3000", &schemas);
    for line in angular_code.lines().take(25) {
        println!("{}", line);
    }
}
