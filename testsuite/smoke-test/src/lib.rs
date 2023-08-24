// Copyright © Diem Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

extern crate core;

#[cfg(test)]
mod diem;
#[cfg(test)]
mod diem_cli;
#[cfg(test)]
mod client;
#[cfg(test)]
mod consensus;
#[cfg(test)]
mod full_nodes;
#[cfg(test)]
mod fullnode;
#[cfg(test)]
mod genesis;
#[cfg(test)]
mod indexer;
#[cfg(test)]
mod inspection_service;
#[cfg(test)]
mod network;
#[cfg(test)]
mod rest_api;
#[cfg(test)]
mod rosetta;
#[cfg(test)]
mod state_sync;
#[cfg(test)]
mod storage;
#[cfg(test)]
mod test_smoke_tests;
#[cfg(test)]
mod transaction;
#[cfg(test)]
mod txn_broadcast;
#[cfg(test)]
mod txn_emitter;
#[cfg(test)]
mod upgrade;

// #[cfg(test)]

//////// 0L ////////
// this needs to be callable in other test environments of third party testsuites
pub mod smoke_test_environment;
/////// end 0L ////////

#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod workspace_builder;
