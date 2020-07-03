use lalrpop;

fn main() {

    //lalrpop::process_root().unwrap();

    lalrpop::Configuration::new().log_verbose().process_current_dir().unwrap();
}
