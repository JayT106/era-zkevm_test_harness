// This file is auto-generated, do not edit it manually

use crate::toolset::GeometryConfig;

pub const fn get_geometry_config() -> GeometryConfig {
    GeometryConfig {
    cycles_per_vm_snapshot: 1428,
    cycles_per_code_decommitter_sorter: 12043,
    cycles_per_log_demuxer: 6361,
    cycles_per_storage_sorter: 4972,
    cycles_per_events_or_l1_messages_sorter: 5545,
    limit_for_l1_messages_merklizer: 64,
    cycles_per_ram_permutation: 16247,
    cycles_per_code_decommitter: 768,
    cycles_per_storage_application: 7,
    limit_for_initial_writes_pubdata_hasher: 297,
    limit_for_repeated_writes_pubdata_hasher: 472,
    cycles_per_keccak256_circuit: 133,
    cycles_per_sha256_circuit: 737,
    cycles_per_ecrecover_circuit: 4,
    limit_for_l1_messages_pudata_hasher: 64,
    }
}