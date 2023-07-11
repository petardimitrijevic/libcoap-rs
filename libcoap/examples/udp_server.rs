use std::{
    net::UdpSocket,
    sync::atomic::AtomicU32,
    time::{Duration, Instant},
};

use libcoap_rs::{
    coap_set_log_level,
    message::{CoapMessageCommon, CoapRequest, CoapResponse},
    protocol::{CoapRequestCode, CoapResponseCode},
    session::CoapServerSession,
    CoapContext, CoapLogLevel, CoapRequestHandler, CoapResource,
};

fn main() {
    coap_set_log_level(CoapLogLevel::COAP_LOG_DTLS_BASE);

    let server_address = UdpSocket::bind("127.0.0.1:5683")
        .expect("Failed to bind server socket")
        .local_addr()
        .expect("Failed to get server socket address");

    // a new CoAP context and bind to the generated SocketAddr.
    let mut context = CoapContext::new().expect("Failed to create CoAP context");
    context
        .add_endpoint_udp(server_address)
        .expect("Unable to add/bind to endpoint");

    let handler = CoapRequestHandler::new_resource_ref(
        move |resource: &CoapResource<AtomicU32>,
              session: &mut CoapServerSession,
              request: &CoapRequest,
              response: &mut CoapResponse| {

            let val = request.observe().unwrap();
            println!("Observe {:?} {:?}", request.observe(), request);

            // Set content of the response message to "Hello World!"
            let data = Vec::<u8>::from("world".as_bytes());
            response.set_data(Some(data));

            // Set the response code to 2.00 "Content"
            response.set_code(CoapResponseCode::Content);
        },
    );

    // Create a new resource that is available at the URI path `hello_world`
    // The second argument can be used to provide any kind of user-specific data, which will
    // then be passed to the handler function.
    let resource = CoapResource::new("hello", AtomicU32::new(1), false);
    // Set a method handler for the GET method.
    resource.set_method_handler(CoapRequestCode::Get, Some(handler));
    // resource.set_observe_notify_confirmable(true);
    resource.set_get_observable(true);

    // Add the resource to the context.
    let resource_id = context.add_resource(resource);
    println!("Resource ID {}", resource_id);

    println!("Starting server...");

    let mut now = Instant::now();
    let interval = Duration::new(2, 0);

    loop {
        // process IO in a loop...
        if let Err(e) = context.do_io(Some(Duration::from_secs(1))) {
            break;
        }

        if now + interval < Instant::now() {
            // println!("Notify observers...");
            now = Instant::now();
            context.notify_observers(resource_id);
        }

        // ...until we want to shut down.
    }
    // Properly shut down, completing outstanding IO requests and properly closing sessions.
    context.shutdown(Some(Duration::from_secs(0))).unwrap();
}
