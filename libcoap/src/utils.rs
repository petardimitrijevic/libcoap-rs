#[allow(non_camel_case_types)]
pub enum CoapLogLevel {
    COAP_LOG_EMERG = 0,
    COAP_LOG_ALERT,
    COAP_LOG_CRIT,
    COAP_LOG_ERR,
    COAP_LOG_WARN,
    COAP_LOG_NOTICE,
    COAP_LOG_INFO,
    COAP_LOG_DEBUG,
    COAP_LOG_OSCORE,
    COAP_LOG_DTLS_BASE,
}

pub fn coap_set_log_level(level: CoapLogLevel) {
    unsafe {
        libcoap_sys::coap_set_log_level(level as i32);
    }
}
