#[derive(Clone)]
enum Person{
    Name(String),
    Age(u8),
    City(String)
}

pub fn vectors(){
    let  person_name = vec![
        Person::Name(String::from("Alice")),
        Person::Age(25),
        Person::City(String::from("New York")),
    ];

    // Iterate through the vector and print each element
    for i in &person_name{
        match i {
            Person::Name(name) => println!("Name: {}", name),
            Person::Age(age) => println!("Age: {}", age),
            Person::City(city) => println!("City: {}", city),
        }
    }
}