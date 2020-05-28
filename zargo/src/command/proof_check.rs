//!
//! The `proof-check` command.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::directory::build::Directory as BuildDirectory;
use crate::directory::build::Error as BuildDirectoryError;
use crate::directory::data::Directory as DataDirectory;
use crate::directory::data::Error as DataDirectoryError;
use crate::directory::source::Directory as SourceDirectory;
use crate::executable::compiler::Compiler;
use crate::executable::compiler::Error as CompilerError;
use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;
use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "Runs the full project building, running, trusted setup, proving & verifying sequence"
)]
pub struct Command {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbosity: usize,

    #[structopt(
        long = "manifest-path",
        help = "Path to Zargo.toml",
        default_value = "./Zargo.toml"
    )]
    manifest_path: PathBuf,

    #[structopt(long = "build", help = "Path to the binary data file")]
    binary_path: PathBuf,

    #[structopt(long = "witness", help = "Path to the witness JSON file")]
    witness_path: PathBuf,

    #[structopt(long = "public-data", help = "Path to the public data JSON file")]
    public_data_path: PathBuf,

    #[structopt(long = "proving-key", help = "Path to the proving key file")]
    proving_key_path: PathBuf,

    #[structopt(long = "verifying-key", help = "Path to the verifying key file")]
    verifying_key_path: PathBuf,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(ManifestError),
    #[fail(display = "build directory {}", _0)]
    BuildDirectory(BuildDirectoryError),
    #[fail(display = "data directory {}", _0)]
    DataDirectory(DataDirectoryError),
    #[fail(display = "compiler {}", _0)]
    Compiler(CompilerError),
    #[fail(display = "virtual machine 'run' {}", _0)]
    VirtualMachineRun(VirtualMachineError),
    #[fail(display = "virtual machine 'setup' {}", _0)]
    VirtualMachineSetup(VirtualMachineError),
    #[fail(display = "virtual machine 'prove & verify' {}", _0)]
    VirtualMachineProveAndVerify(VirtualMachineError),
}

impl Command {
    pub fn execute(self) -> Result<(), Error> {
        let _manifest = Manifest::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let source_directory_path = SourceDirectory::path(&manifest_path);
        let build_directory_path = BuildDirectory::path(&manifest_path);
        let data_directory_path = DataDirectory::path(&manifest_path);

        BuildDirectory::create(&manifest_path).map_err(Error::BuildDirectory)?;
        DataDirectory::create(&manifest_path).map_err(Error::DataDirectory)?;

        Compiler::build(
            self.verbosity,
            &data_directory_path,
            &build_directory_path,
            &source_directory_path,
        )
        .map_err(Error::Compiler)?;

        VirtualMachine::run(
            self.verbosity,
            &self.binary_path,
            &self.witness_path,
            &self.public_data_path,
        )
        .map_err(Error::VirtualMachineRun)?;

        VirtualMachine::setup(
            self.verbosity,
            &self.binary_path,
            &self.proving_key_path,
            &self.verifying_key_path,
        )
        .map_err(Error::VirtualMachineSetup)?;

        VirtualMachine::prove_and_verify(
            self.verbosity,
            &self.binary_path,
            &self.witness_path,
            &self.public_data_path,
            &self.proving_key_path,
            &self.verifying_key_path,
        )
        .map_err(Error::VirtualMachineProveAndVerify)?;

        Ok(())
    }
}