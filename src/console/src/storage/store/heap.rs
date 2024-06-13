use crate::storage::types::state::{AssetsHeap, StorageHeapState};
use crate::STATE;
use junobuild_collections::msg::COLLECTION_NOT_FOUND;
use junobuild_collections::types::rules::{Memory, Rule};
use junobuild_shared::types::core::CollectionKey;
use junobuild_storage::types::config::StorageConfig;
use junobuild_storage::types::state::FullPath;
use junobuild_storage::types::store::Asset;

pub fn get_public_asset(full_path: FullPath, token: Option<String>) -> Option<(Asset, Memory)> {
    let asset = get_asset(&full_path);

    match asset {
        None => None,
        Some(asset) => match &asset.key.token {
            None => Some((asset.clone(), Memory::Heap)),
            Some(asset_token) => {
                let protected_asset = get_token_protected_asset(&asset, asset_token, token);
                protected_asset.map(|protected_asset| (protected_asset, Memory::Heap))
            }
        },
    }
}

// TODO: same code as in satellite
fn get_token_protected_asset(
    asset: &Asset,
    asset_token: &String,
    token: Option<String>,
) -> Option<Asset> {
    match token {
        None => None,
        Some(token) => {
            if &token == asset_token {
                return Some(asset.clone());
            }

            None
        }
    }
}

pub fn get_asset(full_path: &FullPath) -> Option<Asset> {
    STATE.with(|state| get_asset_impl(full_path, &state.borrow().heap.get_storage().assets))
}

fn get_asset_impl(full_path: &FullPath, assets: &AssetsHeap) -> Option<Asset> {
    let value = assets.get(full_path);
    value.cloned()
}

/// Rules

// TODO: almost same as satellite except get_storage()

pub fn get_rule(collection: &CollectionKey) -> Result<Rule, String> {
    let rule = STATE.with(|state| {
        let rules = &state.borrow().heap.get_storage().rules.clone();
        let rule = rules.get(collection);

        rule.cloned()
    });

    match rule {
        None => Err([COLLECTION_NOT_FOUND, collection].join("")),
        Some(rule) => Ok(rule),
    }
}

///
/// Config
///

// TODO: almost same as satellite except get_storage()

pub fn get_config() -> StorageConfig {
    STATE.with(|state| state.borrow().heap.get_storage().config.clone())
}

pub fn insert_config(config: &StorageConfig) {
    STATE.with(|state| insert_config_impl(config, &mut state.borrow_mut().heap.storage))
}

fn insert_config_impl(config: &StorageConfig, state: &mut Option<StorageHeapState>) {
    match state {
        Some(state) => {
            state.config = config.clone();
        }
        None => {
            *state = Some(StorageHeapState {
                config: config.clone(),
                ..Default::default()
            });
        }
    }
}
