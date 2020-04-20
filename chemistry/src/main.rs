fn main() {
let mut number1 = 1;

let mut number2 = 2;

for i in 0..5{

// 1 2 3 5 8

let mut result = number1 + number2;

number1 = number2;
number2 = result;

// 1 + 2 = 3
// 1 2 3

println!("{}" ,result);
}
} 
