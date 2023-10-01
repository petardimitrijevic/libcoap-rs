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

impl TryFrom<u32> for CoapLogLevel {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use CoapLogLevel::*;

        Ok(match value {
            0 => COAP_LOG_EMERG,
            1 => COAP_LOG_ALERT,
            2 => COAP_LOG_CRIT,
            3 => COAP_LOG_ERR,
            4 => COAP_LOG_WARN,
            5 => COAP_LOG_NOTICE,
            6 => COAP_LOG_INFO,
            7 => COAP_LOG_DEBUG,
            8 => COAP_LOG_OSCORE,
            9.. => COAP_LOG_DTLS_BASE,
        })
    }
}
