use std::io;
use std::fs::File;
use std::io::BufRead;

fn main() {
    // println!("Hello, world!");
    

    //Different modules in the Introductory course 

    //primitives();
    //conditions_and_loops();
    // functions();
    //structs();
    //strings_vectors();
    // working_with_enums();
    // debugging();

}


//Primitives
fn primitives(){
    let message = "This is a message";

    let mut message2 = String::from("Second message!");
    println!("{}", message2);

    message2 = String::from("Changed second!");
    println!("{}", message2);


    let weight = 150.0;

    let mut kilos = weight/2.2;

    let mut num = 45;

    println!("{} Kilos is {} {}", message, kilos, num);

    kilos = 20.0;
    num = 20;

    println!("Kilos is {} {}", kilos, num);

}

//Conditions and Control Flows
fn conditions_and_loops(){

    let mut x = 0;

    // Kind of a While True
    loop{
        x+=1;
        if x == 2 {continue}
        println!("{}", x);
        if x > 4 {break}
    }

    let mut input = String::new();
    while input.trim() != "stop"{
        input.clear();
        println!("Give me your next input: \n");
        io::stdin().read_line(&mut input).expect("Failed to read input!");
    }

    for x in 0..=10{
        if x % 2 ==0{
            println!("even")
        }
        else{
            println!("odd {}", x)
        }
    }

    for i in (1..=10).rev(){
        print!("{} ", i)
    }

    let numbers = vec![1,2,3,4,5];
    for num in numbers{
        print!("1")
    }

    // Kind of a Switch/Case
    let name = "Jack";

    match name{
        "Jack" => println!("Jack"),
        "Paul" => println!("Paul"),
        _ => println!("Default"),
    }

}

//Functions, Panic, and Error Handling
fn functions(){
    // process_numbers(&[1,2,5]);

    // println!("{}",split_string("hello,hi, how,are,you,".to_string(), ',', 3));
    
    // loop_and_panic();

    // error_catching();
}
fn process_numbers(numbers: &[i32]){
    let mut sum = 0;

    for el in numbers{
        sum += el;
    }

    println!("{}", sum)
}
fn split_string(s: String, deli: char, field: usize) -> String{
    let parts: Vec<&str> = s.split(deli).collect();
    let res = parts.get(field);

    res.expect("Something Bad Happened!").to_string()

}
fn loop_and_panic(){
    let new_vec = vec![1,2,3,4, -5];

    for el in new_vec{
        if el < 0{
            panic!("Panic for element < 0");
        }
        print!("{} ", el)
    }
}
fn error_catching(){
    let file = File::open("no_file.txt");
    let file = match file{
        Ok(file) => file, 
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("No file {}", error);
                }
                _ => {panic!("Error: {}", error)},
            }
        }
    };

    let reader = std::io::BufReader::new(file);
    for line in reader.lines(){
        println!("{}", line.unwrap())
    }

}

//Structs
#[derive(Debug)]
struct Person{
    first_name: String,
    last_name: String,
    age: u8,
}
impl Person{
    fn new(fname: String, lname:String, age: u8) -> Person {
        Person{first_name: fname, last_name: lname, age: age}
    }

    fn get_older(&mut self) {
        self.age += 1;
    }
    
}
#[derive(Debug)]
struct Dog{
    name: String,
    age: Option<u8>,
    owner: String
}
struct Points(i32, i32, i32);
impl Points{
    fn dist(&self) -> f64 {
        let sum: f64 = (self.0^2+self.1^2+self.2^2).into();
        sum.sqrt()
    }
}
fn structs(){
    let john_green = Person{
        first_name : "John".to_string(),
        last_name : "Green".to_string(),
        age : 28
    };

    let mut alex = Person::new("Alex".to_string(), "Morgan".to_string(), 32);
    alex.get_older();
    alex.get_older();
    println!("{}", alex.age);
    println!("{:?}", alex);
    println!("{:?}", john_green);
    println!("{}", john_green.first_name);

    let rex = Dog{
        name : "Rex".to_string(), 
        age : None,
        owner : "Alex".to_string()
    };

    println!("{:?}", rex);

    let my_point = Points(5, 3, 2);
    println!("{}", my_point.0);
    println!("{}", my_point.dist());

}

//Strings and Vectors
fn strings_vectors(){
    let s = "Hello World!";
    print_str(s);

    let mut salut = String::from("Hello 2!");
    print_String(salut);

    string_manipulation()
}
fn print_str(s: &str){
    println!("{}", s);
}
fn print_String(mut s: String){
    s.push_str("No no");
    println!("{}", s);
}
fn string_manipulation(){
    let sentence = "the quick brown fox jumps over a lazy dog".to_string();

    let desc = format!("Title: {}", sentence);
    println!("{}", desc);

    let mut num = 0;
    for c in desc.chars(){
        num += 1
    }
    println!("{}", num);

    let words : Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}",words);
}


// Enums and Variants
enum DiskType{
    SSD,
    HDD
}
#[derive(Debug)]
enum DiskSize{
    KB(u32),
    MB(u32),
    GB(u32),
    TB(u32)
}
impl DiskSize{
    fn show_size(&self){
        match self{
            DiskSize::KB(x) => println!("{} KiloBytes", x),
            DiskSize::MB(x) => println!("{} MegaBytes", x),
            DiskSize::GB(x) => println!("{} GigaBytes", x),
            DiskSize::TB(x) => println!("{} TeraBytes", x),
        }
    }
}
fn working_with_enums(){
    let disktype = DiskType::SSD;
    let disksize = DiskSize::GB(2);

    println!("{:?}", disksize);
    disksize.show_size();
}


// Learning how to debug
fn debugging(){
    let mut input = String::new();
    while input.trim() != "stop"{
        input.clear();
        println!("Please enter and input (enter 'stop' to exit):");
        io::stdin().read_line(&mut input).expect("Failed to read the input!");
        println!("You entered: '{}'", input);
    }
    println!("Goodbye!");
}