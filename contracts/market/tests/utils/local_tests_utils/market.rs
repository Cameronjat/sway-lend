use std::fs;
use std::{collections::HashMap, str::FromStr};

use super::oracle::OracleContract;
use crate::utils::local_tests_utils::oracle::{get_oracle_contract_instance, oracle_abi_calls};
use crate::utils::local_tests_utils::*;
use fuels::prelude::{
    abigen, Contract, DeployConfiguration, StorageConfiguration, TxParameters, BASE_ASSET_ID,
};
use fuels::programs::call_response::FuelCallResponse;
use fuels::test_helpers::{launch_custom_provider_and_get_wallets, WalletsConfig};
use fuels_types::Address;
use rand::Rng;

abigen!(Contract(
    name = "MarketContract",
    abi = "out/debug/market-abi.json"
));

// const TX_PARAMS: TxParameters = TxParameters::default()
// {
//     gas_price: 1,
//     gas_limit: 100_000_000,
//     maturity: 0,
// };

// TODO: Make it a class not to pass a contract instance in the arguments
pub mod market_abi_calls {

    use fuels::prelude::{CallParameters, SettableContract};
    use fuels_types::Address;

    use super::{abigen_bindings::market_contract_mod::AssetConfig, *};

    pub async fn debug_increment_timestamp(market: &MarketContract) -> FuelCallResponse<()> {
        let res = market.methods().debug_increment_timestamp().call().await;
        res.unwrap()
    }

    pub async fn initialize(
        contract: &MarketContract,
        config: &MarketConfiguration,
        assets: &Vec<AssetConfig>,
        step: Option<u64>,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        contract
            .methods()
            .initialize(config.clone(), assets.clone(), step)
            .tx_params(TxParameters::default().set_gas_price(1))
            .call()
            .await
    }

    pub async fn supply_base(
        market: &MarketContract,
        base_asset_id: AssetId,
        amount: u64,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let call_params = CallParameters::default()
            .set_amount(amount)
            .set_asset_id(base_asset_id);

        let tx_params = TxParameters::default()
            .set_gas_limit(100_000_000)
            .set_gas_price(1);

        market
            .methods()
            .supply_base()
            .tx_params(tx_params)
            .call_params(call_params)
            .unwrap()
            .call()
            .await
    }

    pub async fn withdraw_base(
        market: &MarketContract,
        contract_ids: &[&dyn SettableContract],
        amount: u64,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let tx_params = TxParameters::default()
            .set_gas_limit(100_000_000)
            .set_gas_price(1);
        market
            .methods()
            .withdraw_base(amount)
            .tx_params(tx_params)
            .set_contracts(contract_ids)
            .append_variable_outputs(1)
            .call()
            .await
    }

    pub async fn withdraw_collateral(
        market: &MarketContract,
        contract_ids: &[&dyn SettableContract],
        asset: ContractId,
        amount: u64,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let tx_params = TxParameters::default()
            .set_gas_limit(100_000_000)
            .set_gas_price(1);
        market
            .methods()
            .withdraw_collateral(asset, amount)
            .tx_params(tx_params)
            .set_contracts(contract_ids)
            .append_variable_outputs(1)
            .call()
            .await
    }

    pub async fn supply_collateral(
        market: &MarketContract,
        asset_id: AssetId,
        amount: u64,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let call_params = CallParameters::default()
            .set_amount(amount)
            .set_asset_id(asset_id);
        market
            .methods()
            .supply_collateral()
            .call_params(call_params)
            // .append_variable_outputs(1)
            .unwrap()
            .call()
            .await
    }

    pub async fn get_user_collateral(
        market: &MarketContract,
        address: Address,
        asset: ContractId,
    ) -> u64 {
        let res = market
            .methods()
            .get_user_collateral(address, asset)
            .simulate()
            .await;
        res.unwrap().value
    }

    pub async fn available_to_borrow(
        market: &MarketContract,
        contract_ids: &[&dyn SettableContract],
        address: Address,
    ) -> u64 {
        let res = market
            .methods()
            .available_to_borrow(address)
            // .tx_params(TX_PARAMS)
            .set_contracts(contract_ids)
            .simulate()
            .await;
        res.unwrap().value
    }

    pub async fn get_user_supply_borrow(market: &MarketContract, address: Address) -> (u64, u64) {
        market
            .methods()
            .get_user_supply_borrow(address)
            // .tx_params(TX_PARAMS)
            .simulate()
            .await
            .unwrap()
            .value
    }
    pub async fn get_user_basic(market: &MarketContract, address: Address) -> UserBasic {
        let res = market.methods().get_user_basic(address).simulate().await;
        res.unwrap().value
    }
    pub async fn get_market_basics(market: &MarketContract) -> MarketBasics {
        let res = market.methods().get_market_basics().simulate().await;
        res.unwrap().value
    }
    pub async fn totals_collateral(market: &MarketContract, asset: ContractId) -> u64 {
        let res = market.methods().totals_collateral(asset).simulate().await;
        res.unwrap().value
    }
    pub async fn get_utilization(market: &MarketContract) -> u64 {
        market
            .methods()
            .get_utilization()
            // .tx_params(TX_PARAMS)
            .simulate()
            .await
            .unwrap()
            .value
    }
    pub async fn balance_of(market: &MarketContract, asset_id: ContractId) -> u64 {
        let res = market.methods().balance_of(asset_id).simulate().await;
        res.unwrap().value
    }

    pub async fn pause(
        contract: &MarketContract,
        config: &PauseConfiguration,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        contract.methods().pause(config.clone()).call().await
    }

    pub async fn collateral_value_to_sell(
        market: &MarketContract,
        contract_ids: &[&dyn SettableContract],
        asset: ContractId,
        collateral_amount: u64,
    ) -> u64 {
        let res = market
            .methods()
            .collateral_value_to_sell(asset, collateral_amount);
        res //.tx_params(TX_PARAMS)
            .set_contracts(contract_ids)
            .simulate()
            .await
            .unwrap()
            .value
    }

    pub async fn buy_collateral(
        market: &MarketContract,
        base_asset_id: AssetId,
        amount: u64,
        asset: ContractId,
        min_amount: u64,
        recipient: Address,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        let call_params = CallParameters::default()
            .set_amount(amount)
            .set_asset_id(base_asset_id);
        // let tx_params = TxParameters::default()
        //     .set_gas_limit(100_000_000)
        //     .set_gas_price(1);
        market
            .methods()
            .buy_collateral(asset, min_amount, recipient)
            // .tx_params(tx_params)
            .call_params(call_params)
            .unwrap()
            .estimate_tx_dependencies(None)
            .await
            .unwrap()
            .call()
            .await
    }

    pub async fn absorb(
        market: &MarketContract,
        contract_ids: &[&dyn SettableContract],
        addresses: Vec<Address>,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        market
            .methods()
            .absorb(addresses)
            .set_contracts(contract_ids)
            // .tx_params(TX_PARAMS)
            .call()
            .await
    }

    pub async fn is_liquidatable(
        market: &MarketContract,
        contract_ids: &[&dyn SettableContract],
        address: Address,
    ) -> bool {
        let res = market.methods().is_liquidatable(address);
        res.set_contracts(contract_ids)
            // .tx_params(TX_PARAMS)
            .simulate()
            .await
            .unwrap()
            .value
    }

    pub async fn get_collateral_reserves(market: &MarketContract, asset: ContractId) -> I64 {
        let res = market.methods().get_collateral_reserves(asset);
        res
            // .tx_params(TX_PARAMS)
            .simulate()
            .await
            .unwrap()
            .value
    }
    pub async fn get_reserves(market: &MarketContract) -> I64 {
        let res = market.methods().get_reserves();
        res
            // .tx_params(TX_PARAMS)
            .simulate()
            .await
            .unwrap()
            .value
    }
}

async fn init_wallets() -> Vec<WalletUnlocked> {
    launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(4),                     /* Single wallet */
            Some(1),                     /* Single coin (UTXO) */
            Some(1_000_000_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await
}

pub async fn deploy_market_contract(wallet: &WalletUnlocked) -> MarketContract {
    let mut rng = rand::thread_rng();
    let salt = rng.gen::<[u8; 32]>();
    let storage_path = String::from("./out/debug/market-storage_slots.json");

    let deploy_config: DeployConfiguration = DeployConfiguration::default()
        .set_storage_configuration(StorageConfiguration::default().set_storage_path(storage_path))
        .set_salt(salt)
        .set_tx_parameters(TxParameters::default().set_gas_price(1));

    let id = Contract::deploy("./out/debug/market.bin", &wallet, deploy_config)
        .await
        .unwrap();

    MarketContract::new(id, wallet.clone())
}

pub async fn setup_market() -> (
    Vec<WalletUnlocked>,
    HashMap<String, Asset>,
    MarketContract,
    OracleContract,
) {
    //--------------- WALLET ---------------
    let wallets = init_wallets().await;
    let address = Address::from(wallets[0].address());

    //--------------- ORACLE ---------------
    let oracle_instance = get_oracle_contract_instance(&wallets[0]).await;
    let price_feed = ContractId::from(oracle_instance.contract_id());
    oracle_abi_calls::initialize(&oracle_instance, address).await;
    assert!(oracle_abi_calls::owner(&oracle_instance).await == address);
    // oracle_abi_calls::sync_prices(&oracle_instance, &assets).await;

    //--------------- TOKENS ---------------

    let deploy_config_json_str = fs::read_to_string("tests/utils/local_tests_utils/tokens.json")
        .expect("Should have been able to read the file");
    let deploy_configs: serde_json::Value =
        serde_json::from_str(deploy_config_json_str.as_str()).unwrap();
    let deploy_configs = deploy_configs.as_array().unwrap();
    let mut assets: HashMap<String, Asset> = HashMap::new();
    let mut asset_configs: Vec<AssetConfig> = Vec::new();
    for config_value in deploy_configs {
        let config = DeployTokenConfig {
            name: String::from(config_value["name"].as_str().unwrap()),
            symbol: String::from(config_value["symbol"].as_str().unwrap()),
            decimals: config_value["decimals"].as_u64().unwrap() as u8,
            mint_amount: config_value["mint_amount"].as_u64().unwrap(),
        };

        let instance = if config.symbol != "ETH" {
            Some(token::get_token_contract_instance(&wallets[0], &config).await)
        } else {
            None
        };
        let contract_id = match instance {
            Option::Some(instance) => ContractId::from(instance.contract_id()),
            Option::None => ContractId::from_str(BASE_ASSET_ID.to_string().as_str())
                .expect("Cannot parse BASE_ASSET_ID to contract id"),
        };

        assets.insert(
            String::from(config_value["symbol"].as_str().unwrap()),
            Asset {
                config,
                contract_id,
                asset_id: AssetId::from(*contract_id),
                coingeco_id: String::from(config_value["coingeco_id"].as_str().unwrap()),
                default_price: parse_units(config_value["default_price"].as_u64().unwrap(), 9),
                instance: Option::None,
            },
        );

        if config_value["symbol"].as_str().unwrap() != String::from("USDC") {
            asset_configs.push(AssetConfig {
                asset: contract_id,
                decimals: config_value["decimals"].as_u64().unwrap() as u8,
                price_feed: price_feed,
                borrow_collateral_factor: config_value["borrow_collateral_factor"]
                    .as_u64()
                    .unwrap(), // decimals: 4
                liquidate_collateral_factor: config_value["liquidate_collateral_factor"]
                    .as_u64()
                    .unwrap(), // decimals: 4
                liquidation_penalty: config_value["liquidation_penalty"].as_u64().unwrap(), // decimals: 4
                supply_cap: config_value["supply_cap"].as_u64().unwrap(), // decimals: asset decimals
            })
        }
    }

    //--------------- MARKET ---------------
    let market_instance = deploy_market_contract(&wallets[0]).await;
    let config_json_str = fs::read_to_string("tests/utils/local_tests_utils/config.json")
        .expect("Should have been able to read the file");
    let config: serde_json::Value = serde_json::from_str(config_json_str.as_str()).unwrap();
    let config = config.as_object().unwrap();

    let market_config = get_market_config(
        address,
        address,
        assets.get("USDC").unwrap().contract_id,
        assets.get("USDC").unwrap().config.decimals,
        price_feed,
        assets.get("SWAY").unwrap().contract_id,
    );

    let step = Option::Some(config["debug_step"].as_u64().unwrap());
    market_abi_calls::initialize(&market_instance, &market_config, &asset_configs, step)
        .await
        .expect("❌ Cannot initialize market");

    (wallets, assets, market_instance, oracle_instance)
}

pub fn get_market_config(
    governor: Address,
    pause_guardian: Address,
    base_token: ContractId,
    base_token_decimals: u8,
    price_feed: ContractId,
    reward_token: ContractId,
) -> MarketConfiguration {
    let config_json_str = fs::read_to_string("tests/utils/local_tests_utils/config.json")
        .expect("Should have been able to read the file");
    let config: serde_json::Value = serde_json::from_str(config_json_str.as_str()).unwrap();
    let config = config.as_object().unwrap();

    MarketConfiguration {
        governor,
        pause_guardian,
        base_token,
        base_token_decimals,
        base_token_price_feed: price_feed,
        kink: config["kink"].as_u64().unwrap(), // decimals: 18
        supply_per_second_interest_rate_slope_low: config
            ["supply_per_second_interest_rate_slope_low"]
            .as_u64()
            .unwrap(), // decimals: 18
        supply_per_second_interest_rate_slope_high: config
            ["supply_per_second_interest_rate_slope_high"]
            .as_u64()
            .unwrap(), // decimals: 18
        borrow_per_second_interest_rate_slope_low: config
            ["borrow_per_second_interest_rate_slope_low"]
            .as_u64()
            .unwrap(), // decimals: 18
        borrow_per_second_interest_rate_slope_high: config
            ["borrow_per_second_interest_rate_slope_high"]
            .as_u64()
            .unwrap(), // decimals: 18
        borrow_per_second_interest_rate_base: config["borrow_per_second_interest_rate_base"]
            .as_u64()
            .unwrap(), // decimals: 18
        store_front_price_factor: config["store_front_price_factor"].as_u64().unwrap(), // decimals: 4
        base_tracking_supply_speed: config["base_tracking_supply_speed"].as_u64().unwrap(), // decimals 18
        base_tracking_borrow_speed: config["base_tracking_borrow_speed"].as_u64().unwrap(), // decimals 18
        base_min_for_rewards: config["base_min_for_rewards"].as_u64().unwrap(), // decimals base_token_decimals
        base_borrow_min: config["base_borrow_min"].as_u64().unwrap(), // decimals: base_token_decimals
        target_reserves: config["target_reserves"].as_u64().unwrap(), // decimals: base_token_decimals
        reward_token,
    }
}
