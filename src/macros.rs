macro_rules! respond_with_json_err {
    ($response:expr, $err:expr) => {
        $response.send(format!("{{\"error\":\"{}\"}}", $err))
    };
}
