/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. 
The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

fn compute(n: i64) -> i64{
   (1..n).filter(|&x| x % 3 == 0 || x % 5 == 0).sum()
}

fn main(){
    println!("The sum of all the multiples of 3 or 5 is {}",compute(1000))
}

#[test]
fn test() {
    assert_eq!(compute(10), 23);
    assert_eq!(compute(1000), 233168);
}