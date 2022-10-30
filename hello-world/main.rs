fn main() {
    
    println!("Hello, World!");
    println!("+{:-^8}+",".");
    for _line in 0..8 {
        println!("|{:.>8}|",".");
    }
    println!("+{:-^8}+",".");
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 10u32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => *name="Johny",
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
println!("{:->8}","+");
    for name in names.iter() {
        match name {
        &"Bob" => println!("BOBYYY HELLO"),            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

}