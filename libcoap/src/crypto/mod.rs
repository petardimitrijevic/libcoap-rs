// SPDX-License-Identifier: BSD-2-Clause
/*
 * crypto/mod.rs - CoAP cryptography provider interfaces and types.
 * This file is part of the libcoap-rs crate, see the README and LICENSE files for
 * more information and terms of use.
 * Copyright © 2021-2023 The NAMIB Project Developers, all rights reserved.
 * See the README as well as the LICENSE file for more information.
 */
#[cfg(feature = "dtls")]
pub mod dtls_psk;

#[cfg(feature = "dtls")]
pub use dtls_psk::{
    CoapClientCryptoProvider,
    CoapCryptoProviderResponse, CoapCryptoPskData, CoapCryptoPskIdentity, CoapCryptoPskInfo, CoapServerCryptoProvider,
};
