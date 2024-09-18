fn is_even(n: i32) -> bool{
    n % 2 == 0
}

fn main(){
    let myArr: [i32;10] = [1,2,3,4,5,6,7,8,9,30];

    for &x in myArr.iter(){
        
        if x % 5 == 0 && x % 3 == 0 {
            println!("FizzBuzz");
        }
        else if x % 3 == 0{
            println!("Fizz");
        }
        else if x % 5 == 0{
            println!("Buzz");
        }
        else{
            if is_even(x){
                println!("even");
            }
            else{
                println!("odd");
            }
        }
    }

    let mut i = 0;
    let mut total = 0;
    while i < myArr.len() {
        total = total + myArr[i];
        i += 1;
    }
    println!("Total: {}", total);

    let mut e = 0;
    let mut largest = myArr[0];
    for &x in myArr.iter(){
        if largest < myArr[e]{
            largest = x;
        }
        e += 1
    }
    println!("largest: {}", largest)


}