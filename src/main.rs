use rustats::primary::mean;

fn main() {
    let values = [1.0, 2.0, 3.0, 4.0];

    match mean(&values) {
        Some(avg) => println!("mean: {avg}"),
        None => println!("mean: (empty input)"),
    }
}
