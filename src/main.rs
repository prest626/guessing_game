use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);


loop{
    println!("Please input your guess.");

    let mut guess = String::new(); //mut makes the variable mutable. :: means that new is an associated function of String

    io::stdin()
        .read_line(&mut guess) //.read_line takes waht the user types into standard input and appends that into a string (without overweriting its contents). & indicates that the argument is a reference
        .expect("Failed to read line");
    
    let guess: u32 = match guess.trim().parse() {
        Ok(num)=>num,
        Err(_)=> continue,
    };

    //.trim == if guess is "  35  " .trim makes guess = "35" taking off the spaces at the beggining and end. trim also removes escape characters like \n. 


    println!("You guessed: {guess}");


    match guess.cmp(&secret_number){
            Ordering::Less  => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }   
}
