use std::io;
use std::fs::File;
use std::io::BufRead;

fn main() {
    // println!("Hello, world!");
    

    //Different modules in the Introductory course 

    //primitives();
    //conditions_and_loops();
    // functions();

}



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





