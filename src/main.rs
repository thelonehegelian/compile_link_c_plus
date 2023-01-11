extern "C" {
    fn multiply(x: i32, y: i32) -> i32;
}

fn main() {
    unsafe {
        let num_a = 10;
        let num_b = 42;
        let result = multiply(num_a, num_b);
        println!("The sum of {} and {} is {}", num_a, num_b, result);
    }
}
