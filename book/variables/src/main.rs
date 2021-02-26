fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    let mut z = 5;
    println!("The value of y is: {}", z);
    z = 3;



    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of y is: {}", z);
}