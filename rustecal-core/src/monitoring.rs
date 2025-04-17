//! Runtime monitoring interface to query current system state from eCAL.
//!
//! This module wraps the C API `eCAL_Monitoring_GetMonitoring` and provides
//! a safe Rust API to access a snapshot of the middleware's state.

use crate::core_types::monitoring::{MonitoringSnapshot, ProcessInfo, TopicInfo, ServerInfo, ClientInfo};
use std::ptr;

/// Provides access to eCAL runtime monitoring data.
pub struct Monitoring;

impl Monitoring {
    /// Retrieves a snapshot of the current system state from the eCAL runtime.
    ///
    /// This includes publishers, subscribers, servers, clients, and processes.
    ///
    /// # Returns
    ///
    /// A `MonitoringSnapshot` containing the current runtime information,
    /// or an empty snapshot if retrieval fails.
    pub fn get_snapshot() -> MonitoringSnapshot {
        let mut raw_ptr: *mut rustecal_sys::eCAL_Monitoring_SMonitoring = ptr::null_mut();

        let success = unsafe { rustecal_sys::eCAL_Monitoring_GetMonitoring(&mut raw_ptr) };

        if success != 0 || raw_ptr.is_null() {
            return MonitoringSnapshot {
                processes: vec![],
                publishers: vec![],
                subscribers: vec![],
                servers: vec![],
                clients: vec![],
            };
        }

        let snapshot = unsafe {
            let raw = &*raw_ptr;

            let processes = std::slice::from_raw_parts(raw.processes, raw.processes_length)
                .iter()
                .cloned()
                .map(ProcessInfo::from)
                .collect();

            let publishers = std::slice::from_raw_parts(raw.publishers, raw.publishers_length)
                .iter()
                .cloned()
                .map(TopicInfo::from)
                .collect();

            let subscribers = std::slice::from_raw_parts(raw.subscribers, raw.subscribers_length)
                .iter()
                .cloned()
                .map(TopicInfo::from)
                .collect();

            let servers = std::slice::from_raw_parts(raw.servers, raw.servers_length)
                .iter()
                .cloned()
                .map(ServerInfo::from)
                .collect();

            let clients = std::slice::from_raw_parts(raw.clients, raw.clients_length)
                .iter()
                .cloned()
                .map(ClientInfo::from)
                .collect();

            rustecal_sys::eCAL_Free(raw_ptr as *mut _);

            MonitoringSnapshot {
                processes,
                publishers,
                subscribers,
                servers,
                clients,
            }
        };

        snapshot
    }
}
