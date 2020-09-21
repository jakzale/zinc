//!
//! The Zargo project manager `call` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;

use colored::Colorize;
use failure::Fail;
use reqwest::Client as HttpClient;
use reqwest::Method;
use reqwest::Url;
use serde_json::Value as JsonValue;
use structopt::StructOpt;
use num::BigUint;
use num::Zero;

use zinc_data::CallRequestBody;
use zinc_data::CallRequestQuery;

use zksync::web3::types::Address;
use zksync::web3::types::H256;
use zksync::web3::types::U256;
use zksync::web3::types::H160;
use zksync::zksync_models::node::tx::PackedEthSignature;

use crate::transaction::Transaction;
use crate::transaction::error::Error as TransactionError;
use crate::arguments::command::IExecutable;
use crate::directory::data::Directory as DataDirectory;
use crate::file::arguments::Arguments as ArgumentsFile;
use crate::file::error::Error as FileError;
use crate::file::manifest::project_type::ProjectType;
use crate::file::manifest::Manifest as ManifestFile;

///
/// The Zargo project manager `call` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Builds the project at the given path")]
pub struct Command {
    /// The logging level value, which helps the logger to set the logging level.
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    pub verbosity: usize,

    /// The path to the Zargo project manifest file.
    #[structopt(
        long = "manifest-path",
        help = "Path to Zargo.toml",
        default_value = zinc_const::path::MANIFEST,
    )]
    pub manifest_path: PathBuf,

    /// The network identifier, where the contract resides.
    #[structopt(
        long = "network",
        help = "Sets the network, which is either 'rinkeby', 'ropsten', or 'localhost'",
        default_value = "localhost",
    )]
    pub network: String,

    /// The ID of the published contract.
    #[structopt(long = "id", help = "The ID of the published contract")]
    pub contract_id: u32,

    /// The contract method to call.
    #[structopt(long = "method", help = "The contract method to call")]
    pub method: String,
}

///
/// The Zargo project manager `call` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The invalid network error.
    #[fail(
        display = "network must be either `rinkeby`, `ropsten`, or `localhost`, but found `{}`",
        _0
    )]
    NetworkInvalid(String),
    /// The manifest file error.
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(FileError<toml::de::Error>),
    /// The project is not a contract.
    #[fail(display = "not a contract")]
    NotAContract,
    /// The contract method arguments file error.
    #[fail(display = "arguments file {}", _0)]
    ArgumentsFile(FileError<serde_json::Error>),
    /// The transaction argument is missing.
    #[fail(display = "arguments do not contain the transfer transaction")]
    TransactionArgumentMissing,
    /// The transfer transaction signing error.
    #[fail(display = "transfer transaction signing: {}", _0)]
    TransactionSigning(TransactionError),
    /// The publish HTTP request error.
    #[fail(display = "HTTP request: {}", _0)]
    HttpRequest(reqwest::Error),
    /// The smart contract server failure.
    #[fail(display = "action failed: {}", _0)]
    ActionFailed(String),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        let network = zksync::Network::from_str(self.network.as_str()).map_err(Error::NetworkInvalid)?;

        let manifest = ManifestFile::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        match manifest.project.r#type {
            ProjectType::Contract => {}
            _ => return Err(Error::NotAContract),
        }

        let mut manifest_path = self.manifest_path;
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let data_directory_path = DataDirectory::path(&manifest_path);
        let mut arguments_path = data_directory_path;
        arguments_path.push(format!(
            "{}_{}.{}",
            zinc_const::file_name::WITNESS,
            self.method,
            zinc_const::extension::JSON,
        ));

        let arguments = ArgumentsFile::try_from_path(&arguments_path, self.method.as_str())
            .map_err(Error::ArgumentsFile)?;
        let transaction = arguments.get_tx().ok_or(Error::TransactionArgumentMissing)?;
        let transfer = transaction.try_into_transfer("7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110".to_owned()).map_err(Error::TransactionSigning)?;

        let endpoint_url = format!(
            "{}{}",
            network.to_address(),
            zinc_const::zandbox::CONTRACT_CALL_URL
        );

        eprintln!(
            "     {} method `{}` of the contract `{} v{} with ID {}`",
            "Calling".bright_green(),
            self.method,
            manifest.project.name,
            manifest.project.version,
            self.contract_id,
        );
        let http_client = HttpClient::new();
        let mut http_response = http_client
            .execute(
                http_client
                    .request(
                        Method::POST,
                        Url::parse_with_params(
                            endpoint_url.as_str(),
                            CallRequestQuery::new(self.contract_id, self.method, network),
                        )
                        .expect(zinc_const::panic::DATA_CONVERSION),
                    )
                    .json(&CallRequestBody::new(arguments.inner, transfer))
                    .build()
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )
            .map_err(Error::HttpRequest)?;

        if !http_response.status().is_success() {
            return Err(Error::ActionFailed(format!(
                "HTTP error ({}) {}",
                http_response.status(),
                http_response
                    .text()
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )));
        }

        println!(
            "{}",
            serde_json::to_string_pretty(
                &http_response
                    .json::<JsonValue>()
                    .expect(zinc_const::panic::DATA_CONVERSION)
            )
            .expect(zinc_const::panic::DATA_CONVERSION)
        );

        Ok(())
    }
}
