
fn main() {
    println!("****************** RECURSSIVE FUNCTION OF FACTORIAL******************");
    let mut x = 5;
    println!("before fn call {}", x);
    x = factorial(x, 1);
    let x = x; //*  to make it immutable
    println!("before fn call {}", x);
}

//? RECURSSIVE FUNCTION IN RUST
fn factorial(mut x: i32, mut ans: i32) -> i32 {
    if x == 1 {
        return ans;
    }
    ans = ans * x;
    x = x - 1;
    return factorial(x, ans);
}


