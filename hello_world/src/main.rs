fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn mult(x: i32, y:i32) -> i32 {
    let mut total = 0;

    for _i in 0..(y as usize) {
        total = add(total,x);
    }
    total
}

fn exp(base: i32, power:i32) -> i32 {
    let mut total = 1;

    for _i in 0..(power as usize) {
        total = mult(total,base);
    }
    total
}

fn main() {
    println!("2 + 2 = {}", add(2, 2));
    println!("5 * 2 = {}", mult(5, 2));
    println!("5 ** 2 = {}", exp(5, 2));
    println!("5 + 1/3 = {}", add(5, 1/3));


    let mut x:f32 = 5.0;
    let dan_val:f32 = 1.0/3.0;

    x = x +dan_val;
    println!("5 + 1/3 {}", )

}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_add_fn{
        #[test]
        fn test_add() {
            assert_eq!(add(2, 2), 4);
        }      
    }

    mod test_mult_suite{
        #[test]
        fn test_add_multiple() {
            let test_cases = vec![
                (1, 1, 2),
                (0, 0, 0),
                (-1, 1, 0),
                (100, -50, 50)
            ];
            
            for (a, b, expected) in test_cases {
                assert_eq!(add(a, b), expected, "Failed on input ({}, {})", a, b);
            }
        }
    }


    #[test]
    fn test_mult(){
        assert_eq!(mult(5,2),10);
    }

    #[test]
    fn test_exp(){
        assert_eq!(exp(5,2),25);
        assert_eq!(exp(5,-2),1/25);
        assert!(f:32::abs(0.3333 - exp(3,-1)) < 0.005);
    }

    //assert_eq!
        // output should be equal to this
    //assert_ne!
        // output shouldnt be equal to this
    //assert_
}