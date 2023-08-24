// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

//! Helpers for writing Move tests

use anyhow::Result;
use diem_forge::DiemPublicInfo;
use diem_framework::{BuildOptions, BuiltPackage};
use diem_sdk::transaction_builder::TransactionFactory;
use std::path::PathBuf;

/// New style publishing via `code::publish_package`
pub async fn publish_package(
    info: &mut DiemPublicInfo<'_>,
    move_dir: PathBuf,
) -> Result<TransactionFactory> {
    let package = BuiltPackage::build(move_dir, BuildOptions::default())?;
    let blobs = package.extract_code();
    let metadata = package.extract_metadata()?;
    let payload = diem_cached_packages::diem_stdlib::code_publish_package_txn(
        bcs::to_bytes(&metadata).expect("PackageMetadata has BCS"),
        blobs,
    );
    let txn_factory = info.transaction_factory();
    let publish_txn = info
        .root_account()
        .sign_with_transaction_builder(txn_factory.payload(payload));
    info.client().submit_and_wait(&publish_txn).await?;
    Ok(txn_factory)
}