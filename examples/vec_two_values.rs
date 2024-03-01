#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // Sort people by derived natural order (Name and age)
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]);

    // Sort people by age
    people.sort_by(|a, b| b.age.cmp(&a.age));


    println!("{:?}", people);

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]);

println!("People len {}",people.len());

let colors = vec!["blue", "red", "green"];
    // FROM HERE
    // method 1: access vector elements using vector index
    println!("first color = {}", colors[0]);
    println!("second color = {}", colors[1]);
    println!("third color = {}", colors[2]);

    
        let colors = vec!["blue", "red", "green"];
        
        // method 2: access vector elements using get() method and vector index
        println!("first color = {:?}", colors.get(0));
        println!("second color = {:?}", colors.get(1));
        println!("third color = {:?}", colors.get(2));
    
    // Adding Values to a Vector in Rust
    let mut even_numbers = vec![2, 4, 6, 8, 10];

    
    println!("original vector = {:?}", even_numbers);
    
    // push values at the end of the vector
    even_numbers.push(12);
    even_numbers.push(14);

    
    println!("changed vector = {:?}", even_numbers);


}