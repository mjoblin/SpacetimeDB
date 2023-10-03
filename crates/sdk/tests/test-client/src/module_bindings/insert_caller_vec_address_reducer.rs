// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#[allow(unused)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InsertCallerVecAddressArgs {}

impl Reducer for InsertCallerVecAddressArgs {
    const REDUCER_NAME: &'static str = "insert_caller_vec_address";
}

#[allow(unused)]
pub fn insert_caller_vec_address() {
    InsertCallerVecAddressArgs {}.invoke();
}

#[allow(unused)]
pub fn on_insert_caller_vec_address(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<InsertCallerVecAddressArgs> {
    InsertCallerVecAddressArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertCallerVecAddressArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn once_on_insert_caller_vec_address(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<InsertCallerVecAddressArgs> {
    InsertCallerVecAddressArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertCallerVecAddressArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn remove_on_insert_caller_vec_address(id: ReducerCallbackId<InsertCallerVecAddressArgs>) {
    InsertCallerVecAddressArgs::remove_on_reducer(id);
}