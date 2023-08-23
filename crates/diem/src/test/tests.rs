// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    move_tool::{ArgWithType, FunctionArgType},
    CliResult, Tool,
};
use clap::Parser;
use std::str::FromStr;

/// In order to ensure that there aren't duplicate input arguments for untested CLI commands,
/// we call help on every command to ensure it at least runs
#[tokio::test]
async fn ensure_every_command_args_work() {
    assert_cmd_not_panic(&["diem"]).await;

    assert_cmd_not_panic(&["diem", "account"]).await;
    assert_cmd_not_panic(&["diem", "account", "create", "--help"]).await;
    assert_cmd_not_panic(&["diem", "account", "create-resource-account", "--help"]).await;
    assert_cmd_not_panic(&["diem", "account", "fund-with-faucet", "--help"]).await;
    assert_cmd_not_panic(&["diem", "account", "list", "--help"]).await;
    assert_cmd_not_panic(&["diem", "account", "lookup-address", "--help"]).await;
    assert_cmd_not_panic(&["diem", "account", "rotate-key", "--help"]).await;
    assert_cmd_not_panic(&["diem", "account", "transfer", "--help"]).await;

    assert_cmd_not_panic(&["diem", "config"]).await;
    assert_cmd_not_panic(&["diem", "config", "generate-shell-completions", "--help"]).await;
    assert_cmd_not_panic(&["diem", "config", "init", "--help"]).await;
    assert_cmd_not_panic(&["diem", "config", "set-global-config", "--help"]).await;
    assert_cmd_not_panic(&["diem", "config", "show-global-config"]).await;
    assert_cmd_not_panic(&["diem", "config", "show-profiles"]).await;

    assert_cmd_not_panic(&["diem", "genesis"]).await;
    assert_cmd_not_panic(&["diem", "genesis", "generate-genesis", "--help"]).await;
    assert_cmd_not_panic(&["diem", "genesis", "generate-keys", "--help"]).await;
    assert_cmd_not_panic(&["diem", "genesis", "generate-layout-template", "--help"]).await;
    assert_cmd_not_panic(&["diem", "genesis", "set-validator-configuration", "--help"]).await;
    assert_cmd_not_panic(&["diem", "genesis", "setup-git", "--help"]).await;
    assert_cmd_not_panic(&["diem", "genesis", "generate-admin-write-set", "--help"]).await;

    assert_cmd_not_panic(&["diem", "governance"]).await;
    assert_cmd_not_panic(&["diem", "governance", "execute-proposal", "--help"]).await;
    assert_cmd_not_panic(&["diem", "governance", "generate-upgrade-proposal", "--help"]).await;
    assert_cmd_not_panic(&["diem", "governance", "propose", "--help"]).await;
    assert_cmd_not_panic(&["diem", "governance", "vote", "--help"]).await;

    assert_cmd_not_panic(&["diem", "info"]).await;

    assert_cmd_not_panic(&["diem", "init", "--help"]).await;

    assert_cmd_not_panic(&["diem", "key"]).await;
    assert_cmd_not_panic(&["diem", "key", "generate", "--help"]).await;
    assert_cmd_not_panic(&["diem", "key", "extract-peer", "--help"]).await;

    assert_cmd_not_panic(&["diem", "move"]).await;
    assert_cmd_not_panic(&["diem", "move", "clean", "--help"]).await;
    assert_cmd_not_panic(&["diem", "move", "compile", "--help"]).await;
    assert_cmd_not_panic(&["diem", "move", "compile-script", "--help"]).await;
    assert_cmd_not_panic(&["diem", "move", "download", "--help"]).await;
    assert_cmd_not_panic(&["diem", "move", "init", "--help"]).await;
    assert_cmd_not_panic(&["diem", "move", "list", "--help"]).await;
    assert_cmd_not_panic(&["diem", "move", "prove", "--help"]).await;
    assert_cmd_not_panic(&["diem", "move", "publish", "--help"]).await;
    assert_cmd_not_panic(&["diem", "move", "run", "--help"]).await;
    assert_cmd_not_panic(&["diem", "move", "run-script", "--help"]).await;
    assert_cmd_not_panic(&["diem", "move", "test", "--help"]).await;
    assert_cmd_not_panic(&["diem", "move", "transactional-test", "--help"]).await;
    assert_cmd_not_panic(&["diem", "move", "view", "--help"]).await;

    assert_cmd_not_panic(&["diem", "node"]).await;
    assert_cmd_not_panic(&["diem", "node", "check-network-connectivity", "--help"]).await;
    assert_cmd_not_panic(&["diem", "node", "get-stake-pool", "--help"]).await;
    assert_cmd_not_panic(&["diem", "node", "analyze-validator-performance", "--help"]).await;
    assert_cmd_not_panic(&["diem", "node", "bootstrap-db-from-backup", "--help"]).await;
    assert_cmd_not_panic(&["diem", "node", "initialize-validator", "--help"]).await;
    assert_cmd_not_panic(&["diem", "node", "join-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["diem", "node", "leave-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["diem", "node", "run-local-testnet", "--help"]).await;
    assert_cmd_not_panic(&["diem", "node", "show-validator-config", "--help"]).await;
    assert_cmd_not_panic(&["diem", "node", "show-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["diem", "node", "show-validator-stake", "--help"]).await;
    assert_cmd_not_panic(&["diem", "node", "update-consensus-key", "--help"]).await;
    assert_cmd_not_panic(&[
        "diem",
        "node",
        "update-validator-network-addresses",
        "--help",
    ])
    .await;

    assert_cmd_not_panic(&["diem", "stake"]).await;
    assert_cmd_not_panic(&["diem", "stake", "add-stake", "--help"]).await;
    assert_cmd_not_panic(&["diem", "stake", "increase-lockup", "--help"]).await;
    assert_cmd_not_panic(&["diem", "stake", "initialize-stake-owner", "--help"]).await;
    assert_cmd_not_panic(&["diem", "stake", "set-delegated-voter", "--help"]).await;
    assert_cmd_not_panic(&["diem", "stake", "set-operator", "--help"]).await;
    assert_cmd_not_panic(&["diem", "stake", "unlock-stake", "--help"]).await;
    assert_cmd_not_panic(&["diem", "stake", "withdraw-stake", "--help"]).await;
}

/// Ensure we can parse URLs for args
#[tokio::test]
async fn ensure_can_parse_args_with_urls() {
    let result = ArgWithType::from_str("string:https://diemlabs.com").unwrap();
    matches!(result._ty, FunctionArgType::String);
    assert_eq!(
        result.arg,
        bcs::to_bytes(&"https://diemlabs.com".to_string()).unwrap()
    );
}

async fn assert_cmd_not_panic(args: &[&str]) {
    // When a command fails, it will have a panic in it due to an improperly setup command
    // thread 'main' panicked at 'Command propose: Argument names must be unique, but 'assume-yes' is
    // in use by more than one argument or group', ...

    match run_cmd(args).await {
        Ok(inner) => assert!(
            !inner.contains("panic"),
            "Failed to not panic cmd {}: {}",
            args.join(" "),
            inner
        ),
        Err(inner) => assert!(
            !inner.contains("panic"),
            "Failed to not panic cmd {}: {}",
            args.join(" "),
            inner
        ),
    }
}

async fn run_cmd(args: &[&str]) -> CliResult {
    let tool: Tool = Tool::try_parse_from(args).map_err(|msg| msg.to_string())?;
    tool.execute().await
}
