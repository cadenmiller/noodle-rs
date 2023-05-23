

fn main() 
{
    let something: Noodle = Noodle{ name: String::from("Test"), children: HashMap::new() };

    println!("Some testing: {}", something.name);

    println!("Hello, world!");
}