// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::dag::{
    dag_store::Dag,
    storage::DAGStorage,
    tests::helpers::new_certified_node,
    types::{CertifiedNode, Node},
};
use diem_crypto::HashValue;
use diem_infallible::Mutex;
use diem_types::{epoch_state::EpochState, validator_verifier::random_validator_verifier};
use std::{collections::HashMap, sync::Arc};

pub struct MockStorage {
    node_data: Mutex<HashMap<HashValue, Node>>,
    certified_node_data: Mutex<HashMap<HashValue, CertifiedNode>>,
}

impl MockStorage {
    pub fn new() -> Self {
        Self {
            node_data: Mutex::new(HashMap::new()),
            certified_node_data: Mutex::new(HashMap::new()),
        }
    }
}

impl DAGStorage for MockStorage {
    fn save_node(&self, node: &Node) -> anyhow::Result<()> {
        self.node_data.lock().insert(node.digest(), node.clone());
        Ok(())
    }

    fn save_certified_node(&self, node: &CertifiedNode) -> anyhow::Result<()> {
        self.certified_node_data
            .lock()
            .insert(node.digest(), node.clone());
        Ok(())
    }

    fn get_certified_nodes(&self) -> anyhow::Result<HashMap<HashValue, CertifiedNode>> {
        Ok(self.certified_node_data.lock().clone())
    }

    fn delete_certified_nodes(&self, digests: Vec<HashValue>) -> anyhow::Result<()> {
        for digest in digests {
            self.certified_node_data.lock().remove(&digest);
        }
        Ok(())
    }
}

#[test]
fn test_dag_insertion_succeed() {
    let (signers, validator_verifier) = random_validator_verifier(4, None, false);
    let epoch_state = Arc::new(EpochState {
        epoch: 1,
        verifier: validator_verifier.clone(),
    });
    let storage = Arc::new(MockStorage::new());
    let mut dag = Dag::new(epoch_state, storage);

    // Round 1 - nodes 0, 1, 2 links to vec![]
    for signer in &signers[0..3] {
        let node = new_certified_node(1, signer.author(), vec![]);
        assert!(dag.add_node(node).is_ok());
    }
    let parents = dag
        .get_strong_links_for_round(1, &validator_verifier)
        .unwrap();

    // Round 2 nodes 0, 1, 2 links to 0, 1, 2
    for signer in &signers[0..3] {
        let node = new_certified_node(2, signer.author(), parents.clone());
        assert!(dag.add_node(node).is_ok());
    }

    // Round 3 nodes 1, 2 links to 0, 1, 2
    let parents = dag
        .get_strong_links_for_round(2, &validator_verifier)
        .unwrap();

    for signer in &signers[1..3] {
        let node = new_certified_node(3, signer.author(), parents.clone());
        assert!(dag.add_node(node).is_ok());
    }

    // not enough strong links
    assert!(dag
        .get_strong_links_for_round(3, &validator_verifier)
        .is_none());
}

#[test]
fn test_dag_insertion_failure() {
    let (signers, validator_verifier) = random_validator_verifier(4, None, false);
    let epoch_state = Arc::new(EpochState {
        epoch: 1,
        verifier: validator_verifier.clone(),
    });
    let storage = Arc::new(MockStorage::new());
    let mut dag = Dag::new(epoch_state, storage);

    // Round 1 - nodes 0, 1, 2 links to vec![]
    for signer in &signers[0..3] {
        let node = new_certified_node(1, signer.author(), vec![]);
        assert!(dag.add_node(node.clone()).is_ok());
        // duplicate node
        assert!(dag.add_node(node).is_err());
    }

    let missing_node = new_certified_node(1, signers[3].author(), vec![]);
    let mut parents = dag
        .get_strong_links_for_round(1, &validator_verifier)
        .unwrap();
    parents.push(missing_node.certificate());

    let node = new_certified_node(2, signers[0].author(), parents.clone());
    // parents not exist
    assert!(dag.add_node(node).is_err());

    let node = new_certified_node(3, signers[0].author(), vec![]);
    // round too high
    assert!(dag.add_node(node).is_err());

    let node = new_certified_node(2, signers[0].author(), parents[0..3].to_vec());
    assert!(dag.add_node(node).is_ok());
    let node = new_certified_node(2, signers[0].author(), vec![]);
    // equivocation node
    assert!(dag.add_node(node).is_err());
}

#[test]
fn test_dag_recover_from_storage() {
    let (signers, validator_verifier) = random_validator_verifier(4, None, false);
    let epoch_state = Arc::new(EpochState {
        epoch: 1,
        verifier: validator_verifier.clone(),
    });
    let storage = Arc::new(MockStorage::new());
    let mut dag = Dag::new(epoch_state.clone(), storage.clone());

    let mut digests = vec![];

    for round in 1..10 {
        let parents = dag
            .get_strong_links_for_round(round, &validator_verifier)
            .unwrap_or_default();
        for signer in &signers[0..3] {
            let node = new_certified_node(round, signer.author(), parents.clone());
            digests.push(node.digest());
            assert!(dag.add_node(node).is_ok());
        }
    }
    let new_dag = Dag::new(epoch_state, storage.clone());

    for digest in &digests {
        assert!(new_dag.exists(digest));
    }

    let new_epoch_state = Arc::new(EpochState {
        epoch: 2,
        verifier: validator_verifier,
    });

    let _new_epoch_dag = Dag::new(new_epoch_state, storage.clone());
    assert!(storage.certified_node_data.lock().is_empty());
}