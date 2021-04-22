//! Banksy chain configurations.

use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sp_core::{Pair, Public, sr25519};
use sp_runtime::traits::{IdentifyAccount, Verify};

use rococo_parachain_primitives::{AccountId, Signature};
use sc_telemetry::TelemetryEndpoints;

// The URL for the telemetry server.
pub const TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<parachain_runtime::GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
    where
        AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn get_chain_spec(id: ParaId) -> ChainSpec {
    ChainSpec::from_genesis(
        "Banksy Local Testnet",
        "banksy_local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                id,
            )
        },
        vec![],
        None,
        None,
        // Properties
        Some(
            json!({
              "tokenDecimals": 18,
              "tokenSymbol": "BKS"
            })
                .as_object()
                .expect("Provided valid json map")
                .clone(),
        ),
        Extensions {
            relay_chain: "rococo".into(),
            para_id: 1024_u32.into(),
        },
    )
}

pub fn dev_chain_spec(id: ParaId) -> ChainSpec {
    ChainSpec::from_genesis(
        "Banksy Development Testnet",
        "banksy_development_testnet",
        ChainType::Development,
        move || {
            testnet_genesis(
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                id,
            )
        },
        vec![],
        None,
        None,
        // Properties
        Some(
            json!({
              "tokenDecimals": 18,
              "tokenSymbol": "BKS"
            })
                .as_object()
                .expect("Provided valid json map")
                .clone(),
        ),
        Extensions {
            relay_chain: "rococo".into(),
            para_id: 1024_u32.into(),
        },
    )
}

pub fn staging_test_net(id: ParaId) -> ChainSpec {
    ChainSpec::from_genesis(
        "Banksy Staging Testnet",
        "banksy_staging_testnet",
        ChainType::Live,
        move || {
            testnet_genesis(
                hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into(),
                vec![
                    hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into(),
                ],
                id,
            )
        },
        Vec::new(),
        None,
        None,
        // Properties
        Some(
            json!({
              "tokenDecimals": 18,
              "tokenSymbol": "BKS"
            })
                .as_object()
                .expect("Provided valid json map")
                .clone(),
        ),
        Extensions {
            relay_chain: "rococo".into(),
            para_id: 1024_u32.into(),
        },
    )
}

pub fn banksy_testnet_config(id: ParaId) -> ChainSpec {
    ChainSpec::from_genesis(
        "Banksy Rococo PC1",
        "banksy_rococo_pc1",
        ChainType::Live,
        move || {
            testnet_genesis(
                hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into(),
                vec![
                    hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into(),
                ],
                id,
            )
        },
		vec![
			"/ip4/119.45.201.48/tcp/30331/p2p/12D3KooWBmhB3UcRyW377TZwyJ8BcDXjP9yT946YqL1Nr1tXhrce".parse().unwrap(),
            "/ip4/119.45.201.48/tcp/30332/p2p/12D3KooWPotDCFfPGzBsvhBtJeEvFocktcpP1waKo78VVUPo6MgE".parse().unwrap(),
		],
		TelemetryEndpoints::new(vec![(TELEMETRY_URL.into(), 0)]).ok(),
		Some("banksy-pc1"),
        // Properties
        Some(
            json!({
              "tokenDecimals": 18,
              "tokenSymbol": "BKS"
            })
                .as_object()
                .expect("Provided valid json map")
                .clone(),
        ),
        Extensions {
            relay_chain: "rococo".into(),
            para_id: 1024_u32.into(),
        },
    )
}

fn testnet_genesis(
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
) -> parachain_runtime::GenesisConfig {
    parachain_runtime::GenesisConfig {
        frame_system: parachain_runtime::SystemConfig {
            code: parachain_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
            changes_trie_config: Default::default(),
        },
        pallet_balances: parachain_runtime::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        pallet_sudo: parachain_runtime::SudoConfig { key: root_key },
        parachain_info: parachain_runtime::ParachainInfoConfig { parachain_id: id },
    }
}
