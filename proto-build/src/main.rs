//! Build Neutron proto files. This build script clones the CosmosSDK and Neutron version
//! specified in the COSMOS_SDK_REV and NEUTRON_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{fs, path::PathBuf};

use proto_build::{
    code_generator::{CodeGenerator, CosmosProject},
    git,
};

const COSMOS_SDK_REPO: &str = "https://github.com/neutron-org/cosmos-sdk.git";
const NEUTRON_REPO: &str = "https://github.com/neutron-org/neutron.git";
const WASMD_REPO: &str = "https://github.com/neutron-org/wasmd.git";
const COMETBFT_REPO: &str = "https://github.com/cometbft/cometbft.git";
const IBC_GO_REPO: &str = "https://github.com/cosmos/ibc-go.git";
const ICS23_REPO: &str = "https://github.com/cosmos/ics23.git";
const FEEMARKET_REPO: &str = "https://github.com/skip-mev/feemarket.git";
const SLINKY_REPO: &str = "https://github.com/skip-mev/slinky.git";
const INTERCHAIN_SECURITY_REPO: &str = "https://github.com/cosmos/interchain-security.git";
const ADMIN_MODULE_REPO: &str = "https://github.com/neutron-org/admin-module.git";

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.50.11-neutron";

/// The Neutron commit or tag to be cloned and used to build the proto files
const NEUTRON_REV: &str = "b17ddda1286b2b3eeba52cf24b6a6da49b46edd4";

/// The wasmd commit or tag to be cloned and used to build the proto files
const WASMD_REV: &str = "v0.53.2-neutron";

/// The cometbft commit or tag to be cloned and used to build the proto files
const COMETBFT_REV: &str = "v0.38.17";

/// The ibc-go commit or tag to be cloned and used to build the proto files
const IBC_GO_REV: &str = "v8.5.2";

/// The ics23 commit or tag to be cloned and used to build the proto files
const ICS23_REV: &str = "go/v0.11.0";

const FEEMARKET_REV: &str = "v1.1.1";

const SLINKY_REV: &str = "v1.2.0";

const ADMIN_MODULE_REV: &str = "v2.0.2";

const INTERCHAIN_SECURITY_REV: &str = "v5.1.1";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// Directory where the cosmos-sdk repo is located
const COSMOS_SDK_DIR: &str = "../dependencies/cosmos-sdk/";
/// Directory where the neutron repo is located
const NEUTRON_DIR: &str = "../dependencies/neutron/";
/// Directory where the wasmd repo is located
const WASMD_DIR: &str = "../dependencies/wasmd/";
/// Directory where the cometbft repo is located
const COMETBFT_DIR: &str = "../dependencies/cometbft/";
/// Directory where the ibc-go repo is located
const IBC_GO_DIR: &str = "../dependencies/ibc-go/";
/// Directory where the ics23 repo is located
const ICS23_DIR: &str = "../dependencies/ics23/";
/// Directory where the feemarket repo is located
const FEEMARKET_DIR: &str = "../dependencies/feemarket/";
/// Directory where the slinky repo is located
const SLINKY_DIR: &str = "../dependencies/slinky/";
/// Directory where the interchain-security repo is located
const INTERCHAIN_SECURITY_DIR: &str = "../dependencies/interchain-security/";
/// Directory where the admin-module repo is located
const ADMIN_MODULE_DIR: &str = "../dependencies/admin-module/";

/// A temporary directory for repos storing
const TMP_REPOS_DIR: &str = "./dependencies/";
/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";
/// The directory generated cosmos-sdk proto files go into in this repo
const OUT_DIR: &str = "../packages/neutron-std/src/types/";

pub fn generate() {
    let tmp_repos_dir: PathBuf = TMP_REPOS_DIR.parse().unwrap();
    if tmp_repos_dir.exists() {
        fs::remove_dir_all(tmp_repos_dir.clone()).unwrap();
    }

    git::clone_repo(COSMOS_SDK_REPO, COSMOS_SDK_DIR, COSMOS_SDK_REV);
    git::clone_repo(NEUTRON_REPO, NEUTRON_DIR, NEUTRON_REV);
    git::clone_repo(WASMD_REPO, WASMD_DIR, WASMD_REV);
    git::clone_repo(COMETBFT_REPO, COMETBFT_DIR, COMETBFT_REV);
    git::clone_repo(IBC_GO_REPO, IBC_GO_DIR, IBC_GO_REV);
    git::clone_repo(ICS23_REPO, ICS23_DIR, ICS23_REV);
    git::clone_repo(FEEMARKET_REPO, FEEMARKET_DIR, FEEMARKET_REV);
    git::clone_repo(SLINKY_REPO, SLINKY_DIR, SLINKY_REV);
    git::clone_repo(
        INTERCHAIN_SECURITY_REPO,
        INTERCHAIN_SECURITY_DIR,
        INTERCHAIN_SECURITY_REV,
    );
    git::clone_repo(ADMIN_MODULE_REPO, ADMIN_MODULE_DIR, ADMIN_MODULE_REV);

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let cosmos_project = CosmosProject {
        name: "cosmos".to_string(),
        version: COSMOS_SDK_REV.to_string(),
        project_dir: COSMOS_SDK_DIR.to_string(),
        exclude_mods: vec!["reflection".to_string(), "autocli".to_string()],
    };

    let neutron_project = CosmosProject {
        name: "neutron".to_string(),
        version: NEUTRON_REV.to_string(),
        project_dir: NEUTRON_DIR.to_string(),
        exclude_mods: vec![],
    };

    let wasmd_project = CosmosProject {
        name: "wasmd".to_string(),
        version: WASMD_REV.to_string(),
        project_dir: WASMD_DIR.to_string(),
        exclude_mods: vec![],
    };

    let cometbft_project = CosmosProject {
        name: "tendermint".to_string(),
        version: COMETBFT_REV.to_string(),
        project_dir: COMETBFT_DIR.to_string(),
        exclude_mods: vec![],
    };

    let ibc_project = CosmosProject {
        name: "ibc".to_string(),
        version: IBC_GO_REV.to_string(),
        project_dir: IBC_GO_DIR.to_string(),
        exclude_mods: vec![],
    };

    let ics23_project = CosmosProject {
        name: "ics23".to_string(),
        version: ICS23_REV.to_string(),
        project_dir: ICS23_DIR.to_string(),
        exclude_mods: vec![],
    };

    let feemarket_project = CosmosProject {
        name: "feemarket".to_string(),
        version: FEEMARKET_REV.to_string(),
        project_dir: FEEMARKET_DIR.to_string(),
        exclude_mods: vec![],
    };

    let slinky_project = CosmosProject {
        name: "slinky".to_string(),
        version: SLINKY_REV.to_string(),
        project_dir: SLINKY_DIR.to_string(),
        exclude_mods: vec![],
    };

    let interchain_security_project = CosmosProject {
        name: "interchain-security".to_string(),
        version: INTERCHAIN_SECURITY_REV.to_string(),
        project_dir: INTERCHAIN_SECURITY_DIR.to_string(),
        exclude_mods: vec![],
    };

    let admin_project = CosmosProject {
        name: "admin".to_string(),
        version: ADMIN_MODULE_REV.to_string(),
        project_dir: ADMIN_MODULE_DIR.to_string(),
        exclude_mods: vec![],
    };

    let neutron_code_generator = CodeGenerator::new(
        out_dir,
        tmp_build_dir,
        neutron_project,
        vec![
            cosmos_project,
            wasmd_project,
            cometbft_project,
            ibc_project,
            ics23_project,
            feemarket_project,
            slinky_project,
            interchain_security_project,
            admin_project,
        ],
    );

    neutron_code_generator.generate();

    fs::remove_dir_all(tmp_repos_dir.clone()).unwrap();
}

fn main() {
    pretty_env_logger::init();
    generate();
}
