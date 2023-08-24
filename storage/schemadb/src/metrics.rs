// Copyright © Diem Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use diem_metrics_core::{
    exponential_buckets, register_histogram_vec, register_int_counter_vec, HistogramVec,
    IntCounterVec,
};
use once_cell::sync::Lazy;

pub static DIEM_SCHEMADB_SEEK_LATENCY_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_seek_latency_seconds",
        // metric description
        "Diem schemadb seek latency in seconds",
        // metric labels (dimensions)
        &["cf_name", "tag"],
        exponential_buckets(/*start=*/ 1e-6, /*factor=*/ 2.0, /*count=*/ 22).unwrap(),
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_ITER_LATENCY_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_iter_latency_seconds",
        // metric description
        "Diem schemadb iter latency in seconds",
        // metric labels (dimensions)
        &["cf_name"],
        exponential_buckets(/*start=*/ 1e-6, /*factor=*/ 2.0, /*count=*/ 22).unwrap(),
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_ITER_BYTES: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_iter_bytes",
        // metric description
        "Diem schemadb iter size in bytes",
        // metric labels (dimensions)
        &["cf_name"]
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_GET_LATENCY_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_get_latency_seconds",
        // metric description
        "Diem schemadb get latency in seconds",
        // metric labels (dimensions)
        &["cf_name"],
        exponential_buckets(/*start=*/ 1e-6, /*factor=*/ 2.0, /*count=*/ 22).unwrap(),
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_GET_BYTES: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_get_bytes",
        // metric description
        "Diem schemadb get call returned data size in bytes",
        // metric labels (dimensions)
        &["cf_name"]
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_BATCH_COMMIT_LATENCY_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_batch_commit_latency_seconds",
        // metric description
        "Diem schemadb schema batch commit latency in seconds",
        // metric labels (dimensions)
        &["db_name"],
        exponential_buckets(/*start=*/ 1e-3, /*factor=*/ 2.0, /*count=*/ 20).unwrap(),
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_BATCH_COMMIT_BYTES: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_batch_commit_bytes",
        // metric description
        "Diem schemadb schema batch commit size in bytes",
        // metric labels (dimensions)
        &["db_name"]
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_PUT_BYTES: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_put_bytes",
        // metric description
        "Diem schemadb put call puts data size in bytes",
        // metric labels (dimensions)
        &["cf_name"]
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_DELETES: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!("diem_storage_deletes", "Diem storage delete calls", &[
        "cf_name"
    ])
    .unwrap()
});

pub static DIEM_SCHEMADB_BATCH_PUT_LATENCY_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_batch_put_latency_seconds",
        // metric description
        "Diem schemadb schema batch put latency in seconds",
        // metric labels (dimensions)
        &["db_name"],
        exponential_buckets(/*start=*/ 1e-3, /*factor=*/ 2.0, /*count=*/ 20).unwrap(),
    )
    .unwrap()
});
