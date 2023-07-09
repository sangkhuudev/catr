fn main() {
    if let Err(e) = catr::get_args().and_then(catr::runs) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
