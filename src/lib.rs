use wasi::http::proxy::export;
use wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

export!(Component);

struct Component;

impl wasi::exports::http::incoming_handler::Guest for Component {
    fn handle(_req: IncomingRequest, response_out: ResponseOutparam) {
        let headers = Fields::new();
        let body_msg = "Hello from hello-wasm-components!";

        headers
            .set(&"content-type".to_string(), &[b"text/plain".to_vec()])
            .unwrap();

        headers
            .set(
                &"content-length".to_string(),
                &[body_msg.len().to_string().as_bytes().to_vec()],
            )
            .unwrap();

        let resp = OutgoingResponse::new(headers);

        // Extract the body before moving resp
        let body = resp.body().unwrap();

        // Move resp into set
        ResponseOutparam::set(response_out, Ok(resp));

        let stream = body.write().unwrap();
        stream
            .blocking_write_and_flush(body_msg.as_bytes())
            .unwrap();
        drop(stream);

        OutgoingBody::finish(body, None).unwrap();
    }
}
