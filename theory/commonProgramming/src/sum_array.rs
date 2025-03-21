fn sum_even_numbers(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in numbers.iter() {
        if num % 2 == 0 {
            sum += num;
        }
    }
    sum
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6];
    let result = sum_even_numbers(&numbers);
    println!("Tổng các số chẵn: {}", result);
}
