const TEST_MODULE: &str = "./samples/hello_world/hello_world.sp";

fn main() {
    let src = std::fs::read_to_string(TEST_MODULE).unwrap();

    let tokens = sheeppig::lexer::tokenize(&src);

    println!("{:#?}", tokens);
}
