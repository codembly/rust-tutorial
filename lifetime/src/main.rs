  
// Lifetime in RUST
// Codebly - Rust Tutorial


struct Person {
    name: String,
    age: i32,
}

fn main() {
    let _a:i32 = 10;
    let person_a: Person = Person {
        name: String::from("John Doe"),
        age: 55,
    };

    println!("Name {:?} - Age {:?}", get_name(person_a), _a);
}

fn get_name(person: Person) -> String {
    person.name
}

fn get_age(person: Person) -> i32 {
    person.age
}