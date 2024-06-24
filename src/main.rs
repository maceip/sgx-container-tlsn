
use tracing::debug;

use notary_server::{
    run_server, AuthorizationProperties, LoggingProperties, NotarizationProperties,
    NotaryServerError, NotaryServerProperties, NotarySigningKeyProperties, ServerProperties, TLSProperties,
};

fn get_gramine_server_config() -> NotaryServerProperties {
    NotaryServerProperties {
        server: ServerProperties {
            name: "sgx.tlsnotary.io".to_string(),
            host: "127.0.0.1".to_string(),
            port: 3383,
            html_info: "example html response".to_string(),
        },
        notarization: NotarizationProperties { max_transcript_size: 1 << 14 },
        tls: TLSProperties {
            enabled: true,
            private_key_pem_path: "/data/sealed/tls/notary.key".to_string(),
            certificate_pem_path: "/data/sealed/tls/notary.crt".to_string(),
        },
        notary_key: NotarySigningKeyProperties {
            private_key_pem_path: "/data/sealed/notary/notary.key".to_string(),
            public_key_pem_path: "/data/sealed/notary/notary.pub".to_string(),
        },
        logging: LoggingProperties { level: "DEBUG".to_string(), filter: None },
        authorization: AuthorizationProperties {
            enabled: false,
            whitelist_csv_path: "/data/sealed/auth/whitelist.csv".to_string(),
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), NotaryServerError> {
    let notary_config = get_gramine_server_config();

    let config: NotaryServerProperties = notary_config.clone();

    debug!(?config, "enclave notary server config loaded");

    run_server(&config).await?;

    Ok(())
}
