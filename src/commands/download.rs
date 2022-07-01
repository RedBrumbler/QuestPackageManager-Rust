use std::io::Cursor;

use clap::{Args, Subcommand};
use color_eyre::Result;
use zip::ZipArchive;

use crate::utils::network::get_agent;

#[cfg(target_os = "linux")]
static NINJA_DOWNLOAD: &str = "https://github.com/ninja-build/ninja/releases/latest/download/ninja-linux.zip";

#[cfg(darwin)]
static NINJA_DOWNLOAD: &str = "https://github.com/ninja-build/ninja/releases/latest/download/ninja-mac.zip";

#[cfg(windows)]
static NINJA_DOWNLOAD: &str = "https://github.com/ninja-build/ninja/releases/latest/download/ninja-win.zip";

#[derive(Args, Debug, Clone)]
pub struct Download {
    #[clap(subcommand)]
    pub op: DownloadOperation,
}


#[derive(Subcommand, Debug, Clone)]
pub enum DownloadOperation {
    Ninja
    // TODO: NDK
    // TODO: CMake?
}

pub fn execute_download_operation(download_operation: Download) -> Result<()> {
    let download = download_operation.op;

    let url = match download {
        DownloadOperation::Ninja => NINJA_DOWNLOAD,
    };

    let exe = std::env::current_exe()?;
    let final_path = exe.parent().unwrap();

    // extract if zip
    let response = get_agent().get(url).send()?;

    let buffer = Cursor::new(response.bytes()?);
    // Extract to tmp folder
    ZipArchive::new(buffer)?.extract(&final_path)?;

    println!("Sucessfully downloaded and extracted {:?} into {:?}", download, &final_path);

    Ok(())
}
