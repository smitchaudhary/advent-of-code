use day_12::solutions::get_fencing_price;

fn main() {
    let filename = "input.txt".to_string();

    let (price, discounted_price) = get_fencing_price(&filename);

    println!("The price of fencing is {price}");
    println!("The discounted price is {discounted_price}");
}
