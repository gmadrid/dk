fn run_file() {
    let thing = r#"
    chart = read("foobar.knit")
    padded = pad(chart, 5)
    write(padded)
    "#;

    let ast = dklalr::parse_str(thing);
    println!("THE THING: {:?}", ast);
}

fn main() {
    run_file();
}
