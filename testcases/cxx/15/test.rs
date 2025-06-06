#[cxx::bridge()]
mod test {
    struct Test {
        size: usize,
    }

    extern "Rust" {
        fn test() -> Result<Box<Test>>;
    }   
}       
        
fn test() -> Result<Box<test::Test>> {
    Ok(Box::<test::Test>::new(test::Test{size: 0}))
}