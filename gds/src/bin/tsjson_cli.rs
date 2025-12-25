fn main() {
    use std::io::Read;

    // Accept JSON request from:
    // 1) first CLI arg, else
    // 2) stdin
    let request_json = std::env::args().nth(1).unwrap_or_else(|| {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .expect("failed to read stdin");
        buf
    });

    let response_json = gds::applications::services::tsjson::invoke(request_json);
    print!("{response_json}");
}


