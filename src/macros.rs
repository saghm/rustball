macro_rules! respond_with_json_err {
    ($response:expr, $err:expr) => {
        $response.send(format!("{{\"error\":\"{}\"}}", $err))
    };
}

macro_rules! set_content_type {
    ($response:expr, $mime_type:ident / $subtype:ident) => {
        $response.headers_mut().set(ContentType(content_type!($mime_type / $subtype; Charset = Utf8)))
    };
}
