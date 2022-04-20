use substitution::encrypt;

fn main() {
    println!(
        "{}",
        encrypt(
            String::from("Hello, World!"),
            String::from("pvwabchqrguoefyijsxlmnzdkt")
        )
    );
}