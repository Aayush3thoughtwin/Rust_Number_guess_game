
/***********Guessing game**********/
use std::io;
/*To obtain the user input and then print result as output We need the io(input/output) library into scope.
The io library comes from the standard library (which is known as std).*/

use rand::Rng;
// Importing the rand file to generate the random number.

use std::cmp::Ordering;
// This library help us to compairing data's.

fn main(){
/*
1. fn syntax declare the new function.
2.() parentheses , indicates that there are no parameters.
3. The curly bracket, { , start the body of the function.
*/


/* Step 1. We need to take input from user to guess the number.*/

    println!("*****Welcome in Guess Game*****");
    // Welcome string for play the game.

    println!("Please Enter the number to guess : ");
    //specking user to enter num. for guess.
    /*println! is macro that prints a string to the screen.*/

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {} ",secret_number);

    loop{
        println!("Please input your guess: ");

        let mut guess = String::new();
        /*
        1. By using this line we are creating the place to store the user input.
        2. "let mut guess" will introduce a mutable variable named "guess" and on the other side of equal sign "(=)" is the value that guess is bound to, which is result of calling "String::new", a function that returns a new instance of "String". String is a string type provided by the standard library that is growable.

        3. The "::" syntax in the "::new" line indicates that "new" is associated function of the string type.
        An associate function is implemented on a type, In the case of "let mut guess = String::new();" associate function having "String type".


        4. To summarize, the "let mut gue4ss = String::new();" line has created mutable variable that is currently bound to a new, empty instance of a String.

        5. This line uses "let" statement, which is used to create new variable.
        For Example:

        let rupees = 50;
        This line creating new variable with name rupees and binding it to the value 5.

        In Rust language variable is immutable by default. For makeing the variable mutable we will use "mut" ketword before the name of variable.
        Example:
        let mut rupees = 50;

        Example: let rupees = 50;   //****** immutable ******//
        Example: let mut rupees = 50; //****** mutable ******//
        */
        /* Let's come back to guessing game*/



        // Now we are calling stdin function from the "io module" define at the first line of the game.
        io::stdin()
        // If we hadn't put "use std::io" line at the begning of the game, we could have wrriten this function call as "std::io:stdin" the "stdin" function returns an instance of std::io::stdin, which is a type that represents a handle to the standard input for our terminal.

            .read_line(&mut guess)
            //".read_line(&mut guess)" calls the read_line method on the standard input handle to get input from user.
            .expect("Failed to read line");
            // This line will handle error as input coming from the ".read_line(&mut guess)". If we do not use this line then complier will generates warning.
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=> num,
            Err(_) => continue,
        };

        println!("You guessed : {}",guess);
        // println!() print the set of string and the value which is hold by the guess variable and curly brakets {}.
        

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}