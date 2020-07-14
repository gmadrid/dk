use dklalr::Error;
use fehler::throws;

#[throws]
fn run_file() {
    let thing = r#"
    chart = read("charts/jules-braille.knit")
    padded = pad(chart, 5)
    write(padded, "padded_output.knit")
    "#;

    dklalr::run_string(thing)?;
}

#[throws]
fn main() {
    run_file()?;
}
