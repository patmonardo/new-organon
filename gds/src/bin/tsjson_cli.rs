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

    // Convenience: support batching multiple requests in one process.
    //
    // If the input is a JSON array, we invoke each element in order and return a JSON array
    // of response envelopes. This is useful for examples where state is stored in-process
    // (e.g. the TSJSON in-memory catalog).
    let trimmed = request_json.trim();
    if trimmed.starts_with('[') {
        let parsed: serde_json::Value =
            serde_json::from_str(trimmed).expect("invalid JSON array input");
        let Some(arr) = parsed.as_array() else {
            panic!("expected a JSON array input");
        };

        let mut responses: Vec<serde_json::Value> = Vec::with_capacity(arr.len());
        for req in arr {
            let resp_json = gds::applications::services::tsjson::invoke(req.to_string());
            let resp: serde_json::Value =
                serde_json::from_str(&resp_json).expect("invalid JSON response");
            responses.push(resp);
        }
        print!("{}", serde_json::Value::Array(responses));
        return;
    }

    let response_json = gds::applications::services::tsjson::invoke(request_json);
    print!("{response_json}");
}
