fn main() -> Result<(), std::io::Error> {
    println!("Union size is: {}", std::mem::size_of::<Basic>());
    println!("Enum size is: {}", std::mem::size_of::<Wrapped>());

    Ok(())
}

struct Basic {
    f1: f32,
    f2: f32,
    word: String,
}

struct Wrapped {
    f0: Basic,
}
