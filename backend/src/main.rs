use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use openssl::ssl::{SslConnector, SslMethod};
use openssl::x509::{X509, X509NameRef, X509StoreContext};
use openssl::ocsp::{OcspRequest, OcspResponse, OcspCertStatus};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_openssl::SslStream;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use tokio::time::{timeout, Duration};


#[derive(Deserialize)]
struct DomainRequest{
    domain: String,
}

#[derive(Serialize)]
struct CertificateResponse {
    validity_status: bool,
    expiration_date: String,
    issuer_details: String,
    subject_details: String,
    is_valid_for_domain: bool,
    is_not_self_signed: bool,
    revocation_status: String,
}

#[post("/validate_certificate")]
async fn validate_certificate(data: web::Json<DomainRequest>) -> impl Responder {
    let domain = &data.domain;

    // Fetching SSL Certificate
    let certificate = match fetch_ssl_certificate(domain).await {
        Ok(cert) => cert,
        Err(e) => return HttpResponse::BadRequest().body(format!("Error fetching certificate: {}", e)),
    };

    // Extract metadata and perform validations
    let response = match extract_and_validate(&certificate, domain) {
        Ok(res) => res,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Validation error: {}", e)),
    };

    HttpResponse::Ok().json(response)
}

async fn fetch_ssl_certificate(domain: &str) -> Result<X509, Box<dyn std::error::Error>> {
    let addr = format!("{}:443", domain);
    let addr = addr.to_socket_addrs()?.next().ok_or("Unable to resolve domain")?;
    println!("Connecting to {}", addr);  // Logging the connection address

    let tcp_stream = TcpStream::connect(&addr).await?;
    println!("TCP connection established to {}", addr);  // Logging after TCP connection

    let mut connector_builder = SslConnector::builder(SslMethod::tls())?;
    connector_builder.set_min_proto_version(Some(openssl::ssl::SslVersion::TLS1_2))?;
    // Disable certificate verification for debugging only
    connector_builder.set_verify(openssl::ssl::SslVerifyMode::NONE);
    let connector = connector_builder.build();

    let ssl = connector.configure()?.use_server_name_indication(true).into_ssl(domain)?;

    let mut stream = SslStream::new(ssl, tcp_stream)?;
    let connect_result = timeout(Duration::from_secs(10), Pin::new(&mut stream).connect()).await;

    match connect_result {
        Ok(Ok(())) => println!("SSL handshake completed"),
        Ok(Err(e)) => {
            println!("Error during SSL handshake: {}", e);
            return Err(Box::new(e));
        }
        Err(_) => {
            println!("SSL handshake timed out");
            return Err("SSL handshake timed out".into());
        }
    }

    let cert = stream.ssl().peer_certificate().ok_or("No certificate found")?;
    println!("SSL certificate retrieved for {}", domain);

    Ok(cert)
}





fn extract_and_validate(cert: &X509, domain: &str) -> Result<CertificateResponse, Box<dyn std::error::Error>> {
    // Expiry date
    let not_after = cert.not_after();
    let expiration_date = not_after.to_string();
    let validity_status = not_after > openssl::asn1::Asn1Time::days_from_now(0)?;

    // Issuer and subject details
    let issuer_details = x509_name_ref_to_string(cert.issuer_name());
    let subject_details = x509_name_ref_to_string(cert.subject_name());

    // Valid for domain (Subject Alternative Name)
    let is_valid_for_domain = cert.subject_alt_names()
        .map_or(false, |names| names.iter().any(|name| name.dnsname() == Some(domain)));

    // Self-signed check
    let is_not_self_signed = issuer_details != subject_details;

    // Revocation status - for simplicity, set as "Unknown"
    let revocation_status = "Unknown".to_string();

    // Creating the response
    let response = CertificateResponse {
        validity_status,
        expiration_date,
        issuer_details,
        subject_details,
        is_valid_for_domain,
        is_not_self_signed,
        revocation_status,
    };

    Ok(response)
}

fn x509_name_ref_to_string(name: &X509NameRef) -> String {
    name.entries()
        .map(|e| e.data().as_utf8().unwrap().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .service(validate_certificate)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}