// fn is_palindrome(s: &str) -> bool {
//     let reversed = s.chars().rev().collect::<String>();
//     s == reversed
// }

fn is_palindrome(s){
let reversed=s.chars().rev();
s==reversed;
}
fn main() {
    let test_str:&str = "racecar";
    if is_palindrome(test_str) {
        println!("{} is a palindrome.", test_str);
    } else {
        println!("{} is not a palindrome.", test_str);
    }
}
