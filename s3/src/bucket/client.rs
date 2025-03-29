use std::sync::Arc;

use bytes::Bytes;
use http_body_util::Full;
use hyper_rustls::HttpsConnector;
use hyper_rustls::HttpsConnectorBuilder;
use hyper_timeout::TimeoutConnector;
use hyper_util::client::legacy::{connect::HttpConnector, Client};
use hyper_util::rt::TokioExecutor;
use rustls::client::danger::HandshakeSignatureValid;
use rustls::client::danger::ServerCertVerified;
use rustls::client::danger::ServerCertVerifier;
use rustls::ClientConfig;
use rustls::SignatureScheme;

use crate::{error::S3Error, Bucket};

pub fn create_client(
    request_timeout: Option<std::time::Duration>,
) -> Result<Client<TimeoutConnector<HttpsConnector<HttpConnector>>, Full<Bytes>>, S3Error> {
    let https_connector = HttpsConnectorBuilder::new();

    let https_connector = if cfg!(feature = "no-verify-ssl") {
        https_connector.with_tls_config(get_rustls_config_dangerous()?)
    } else {
        https_connector.with_webpki_roots()
    };

    let https_connector = if cfg!(feature = "http") {
        https_connector.https_or_http().enable_http2().build()
    } else {
        https_connector.https_only().enable_http2().build()
    };

    let mut timeout_connector = TimeoutConnector::new(https_connector);
    timeout_connector.set_connect_timeout(request_timeout);
    timeout_connector.set_read_timeout(request_timeout);
    timeout_connector.set_write_timeout(request_timeout);

    Ok(Client::builder(TokioExecutor::new()).build::<_, Full<Bytes>>(timeout_connector))
}

pub fn get_rustls_config_dangerous() -> Result<ClientConfig, S3Error> {
    let store = rustls::RootCertStore::empty();

    let mut config = ClientConfig::builder()
        .with_root_certificates(store)
        .with_no_client_auth();

    // completely disable cert-verification
    let mut dangerous_config = ClientConfig::dangerous(&mut config);
    dangerous_config.set_certificate_verifier(Arc::new(NoCertificateVerification {}));

    Ok(config)
}
#[derive(Debug)]
pub struct NoCertificateVerification {}

impl ServerCertVerifier for NoCertificateVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        Vec::from([
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::ECDSA_NISTP521_SHA512,
            SignatureScheme::ECDSA_SHA1_Legacy,
            SignatureScheme::ED25519,
            SignatureScheme::ED448,
            SignatureScheme::RSA_PKCS1_SHA1,
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::RSA_PSS_SHA256,
            SignatureScheme::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA512,
        ])
    }
}

impl Bucket {
    pub fn http_client(
        &self,
    ) -> Client<TimeoutConnector<HttpsConnector<HttpConnector>>, Full<Bytes>> {
        self.http_client.clone()
    }
}
