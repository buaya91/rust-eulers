mod solutions;
fn main() {
    let ans1 = solutions::solutions::problem1();
    let ans2 = solutions::solutions::problem2();
    let ans52 = solutions::solutions::problem52();
    println!("
        answer 1: {}
        answer 2: {}
        answer 52: {}
    ", ans1, ans2, ans52);
}
