use super::raw::RawConfig;
use connector_utils::config::{
    AwsKmsConfig, ContractConfig, DeserializeRawConfig, Error, KmsWallet, Result,
};
use std::{
    fmt::{self, Display},
    path::Path,
};
use tracing::info;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub database_pool_size: u32,
    pub gateway_url: String,
    pub chain_id: u64,
    pub decryption_contract: ContractConfig,
    pub kms_management_contract: ContractConfig,
    pub service_name: String,
    pub wallet: KmsWallet,
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Service Name: {}", self.service_name)?;
        writeln!(f, "Database URL: {}", self.database_url)?;
        writeln!(f, "Database connection pool size: {}", self.database_pool_size)?;
        writeln!(f, "Gateway URL: {}", self.gateway_url)?;
        writeln!(f, "Chain ID: {}", self.chain_id)?;
        writeln!(f, "{}", self.decryption_contract)?;
        writeln!(f, "{}", self.kms_management_contract)?;
        writeln!(f, "Wallet address: {:#x}", self.wallet.address())
    }
}

impl Config {
    pub async fn from_env_and_file<P: AsRef<Path>>(path: Option<P>) -> Result<Self> {
        if let Some(p) = &path { info!("Loading config from: {}", p.as_ref().display()); }
        else { info!("Loading config using environment variables only"); }

        let raw_config = RawConfig::from_env_and_file(path)?;
        let config = Self::parse(raw_config).await?;

        info!("Configuration loaded successfully:\n{}", config);
        Ok(config)
    }

    async fn parse(raw_config: RawConfig) -> Result<Self> {
        if raw_config.gateway_url.is_empty() {
            return Err(Error::EmptyField("Gateway URL".into()));
        }

       let wallet = match (raw_config.private_key.as_deref(), raw_config.aws_kms_config) {
            (Some(pk), _) => KmsWallet::from_private_key_str(pk,&Some(raw_config.chain_id))?,
            (None , Some(kms_cfg)) => KmsWallet::from_aws_kms(kms_cfg,&Some(raw_config.chain_id)).await?,
            _ => return Err(Error::InvalidConfig("Either AWS KMS or private key must be configured".into())),
       };

       Ok(Self{
           database_url : raw_config.database_url,
           database_pool_size : raw_config.database_pool_size ,
           gateway_url : raw_config.gateway_url ,
           chain_id : raw_config.chain_id ,
           decryption_contract : ContractConfig::parse("Decryption",raw_config.decryption_contract)? ,
           kms_management_contract : ContractConfig::parse("KmsManagement",raw_config.kms_management_contract)? ,
           service_name : raw_config.service_name ,
           wallet 
       })
   }
}
