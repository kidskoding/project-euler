use project_euler::*;

fn main() {
    println!("prob 1: {}", prob01::prob01());
    println!("prob 2: {}", prob02::prob02());
    println!("prob 3: {}", prob03::prob03());
    println!("prob 4: {}", prob04::prob04());
    println!("prob 5: {}", prob05::prob05());
    println!("prob 6: {}", prob06::prob06());
    println!("prob 7: {}", prob07::prob07());
    println!("prob 8: {}", prob08::prob08());
    println!("prob 9: {:?}", prob09::prob09().expect("expected a numerical signed product for pythagorean triple!"));
    println!("prob 10: {}", prob10::prob10());
}