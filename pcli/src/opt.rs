use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pcli",
    about = "The Penumbra command-line interface.",
    version = env!("VERGEN_GIT_SEMVER"),
)]
pub struct Opt {
    /// The address of the Tendermint node.
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub node: String,
    #[structopt(short, long, default_value = "26657")]
    pub abci_port: u16,
    #[structopt(long, default_value = "26666")]
    pub wallet_port: u16,
    #[structopt(subcommand)]
    pub cmd: Command,
    /// The location of the wallet file [default: platform appdata directory]
    #[structopt(short, long)]
    pub wallet_location: Option<String>,
}

// Note: can't use `Vec<u8>` directly, as structopt would instead look for
// conversion function from `&str` to `u8`.
type Bytes = Vec<u8>;

fn parse_bytestring(s: &str) -> Result<Vec<u8>, String> {
    let decoded = hex::decode(s).expect("Invalid bytestring");

    Ok(decoded)
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Creates a transaction.
    Tx(Tx),
    /// Queries the Penumbra state.
    #[structopt()]
    Query { key: String },
    /// Manages the wallet state.
    Wallet(Wallet),
    /// Manages addresses.
    Addr(Addr),
    /// Synchronizes the chain state to the client.
    Sync,
    /// Fetch transaction by note commitment - TEMP (developer only, remove when sync implemented)
    FetchByNoteCommitment { note_commitment: String },
    /// Block request - TEMP (developer only, remove when sync implemented)
    BlockRequest { start_height: u32, end_height: u32 },
    /// Asset Registry Lookup based on asset ID
    AssetLookup {
        #[structopt(parse(try_from_str = parse_bytestring))]
        asset_id: Bytes,
    },
    /// List every asset in the Asset Registry
    AssetList {},
}

#[derive(Debug, StructOpt)]
pub enum Wallet {
    /// Import an existing spend seed.
    Import,
    /// Generate a new spend seed.
    Generate,
    /// Delete the wallet permanently.
    Delete,
    /// Fetch transaction by note commitment - TEMP (not gonna be exposed to user)
    FetchByNoteCommitment,
}

#[derive(Debug, StructOpt)]
pub enum Addr {
    /// List addresses.
    List,
    /// Show the address with the given index.
    Show {
        /// The index of the address to show.
        #[structopt(short, long)]
        index: u32,
    },
    /// Create a new address.
    New {
        /// A freeform label for the address, stored only locally.
        label: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum Tx {
    /// Send transaction to the node.
    Send {
        /// Amount to send.
        amount: u64,
        /// Denomination.
        denomination: String,
        /// Destination address.
        address: String,
        /// Fee.
        fee: u64,
    },
}
