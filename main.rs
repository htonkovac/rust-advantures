fn main() {
    
    println!("Hello, World!");
    println!("+{:-^8}+",".");
    for _line in 0..8 {
        println!("|{:.>8}|",".");
    }
    println!("+{:-^8}+",".");

}