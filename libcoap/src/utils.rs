use libcoap_sys::coap_log_t;

pub fn coap_startup() {
    unsafe {
        libcoap_sys::coap_startup();
    }
}

pub fn coap_set_log_level(level: coap_log_t) {
    unsafe {
        libcoap_sys::coap_set_log_level(level);
    }
}
