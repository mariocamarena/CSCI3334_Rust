fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        return 0;
    } 
    else if guess > secret {
        return 1;
    } 
    else {
        return -1;
    }
}

fn main() {
    let secret: i32 = 10;
    let mut guess: i32;
    let mut attempts = 0;

    loop {
        guess = 5 + attempts;
        attempts += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!(" secret number: {}", secret);
            break;
        } 
        else if result == 1 {
            println!("high");
        } 
        else {
            println!("low");
        }
    }

    println!("{} guesses ", attempts);
}