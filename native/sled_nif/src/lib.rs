#![warn(clippy::all, clippy::pedantic)]

mod types;
mod utils;

use rustler::{init, nif, Binary, Env, NifResult, Term};

use types::*;
use utils::*;

#[nif]
fn sled_config_new(opts: SledConfigOptions) -> NifResult<SledConfig> {
    Ok(SledConfig::with_opts(opts))
}

#[nif(schedule = "DirtyIo")]
fn sled_config_open(config: SledConfig) -> NifResult<SledDb> {
    rustler_result_from_sled(config.open())
        .map(|db| SledDb::with_db_and_path(db, String::from(config.path.to_string_lossy())))
}

#[nif(schedule = "DirtyIo")]
fn sled_open(path: String) -> NifResult<SledDb> {
    rustler_result_from_sled(sled::open(path.clone())).map(|db| SledDb::with_db_and_path(db, path))
}

#[nif(schedule = "DirtyIo")]
fn sled_db_checksum(db: SledDb) -> NifResult<u32> {
    rustler_result_from_sled(db.checksum())
}

#[nif(schedule = "DirtyIo")]
fn sled_size_on_disk(db: SledDb) -> NifResult<u64> {
    rustler_result_from_sled(db.size_on_disk())
}

#[nif(schedule = "DirtyIo")]
fn sled_was_recovered(db: SledDb) -> bool {
    db.was_recovered()
}

#[nif(schedule = "DirtyIo")]
fn sled_tree_open(db: SledDb, name: String) -> NifResult<SledTree> {
    rustler_result_from_sled(db.open_tree(name.clone()))
        .map(|tree| SledTree::with_tree_db_and_name(tree, db, name))
}

#[nif(schedule = "DirtyIo")]
fn sled_tree_drop(db: SledDb, name: String) -> NifResult<bool> {
    rustler_result_from_sled(db.drop_tree(name))
}

#[nif(schedule = "DirtyIo")]
fn sled_tree_names(env: Env, db: SledDb) -> NifResult<Vec<Binary>> {
    let tree_names = db.tree_names();
    let mut result = Vec::with_capacity(tree_names.len());

    for tree_name in tree_names {
        result.push(try_binary_from_ivec(env, &tree_name)?)
    }

    Ok(result)
}

#[nif(schedule = "DirtyIo")]
fn sled_checksum(tree: SledDbTree) -> NifResult<u32> {
    rustler_result_from_sled(tree.checksum())
}

#[nif(schedule = "DirtyIo")]
fn sled_flush(tree: SledDbTree) -> NifResult<usize> {
    rustler_result_from_sled(tree.flush())
}

#[nif(schedule = "DirtyIo")]
fn sled_insert<'a>(
    env: Env<'a>,
    tree: SledDbTree,
    k: Binary,
    v: Binary,
) -> NifResult<Option<Binary<'a>>> {
    try_binary_result_from_sled(env, tree.insert(&k[..], &v[..]))
}

#[nif(schedule = "DirtyIo")]
fn sled_get<'a>(env: Env<'a>, tree: SledDbTree, k: Binary) -> NifResult<Option<Binary<'a>>> {
    try_binary_result_from_sled(env, tree.get(&k[..]))
}

#[nif(schedule = "DirtyIo")]
fn sled_remove<'a>(env: Env<'a>, tree: SledDbTree, k: Binary) -> NifResult<Option<Binary<'a>>> {
    try_binary_result_from_sled(env, tree.remove(&k[..]))
}

fn on_load(env: Env, _info: Term) -> bool {
    types::on_load(env)
}

init! {
    "Elixir.Sled.Native",
    [
        sled_config_new,
        sled_config_open,
        sled_open,
        sled_tree_open,
        sled_tree_drop,
        sled_tree_names,
        sled_db_checksum,
        sled_size_on_disk,
        sled_was_recovered,
        sled_checksum,
        sled_flush,
        sled_insert,
        sled_get,
        sled_remove
    ],
    load = on_load
}
