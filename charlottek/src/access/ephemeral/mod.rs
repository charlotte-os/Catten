//! # Ephemeral Capabilities
//!
//! In `charlottek`, ephemeral capabilities are keys for descriptors in an ephemeral capability
//! table. There exists one such capability table per isolation domain. Each descriptor is an ADT
//! whose variant represents the type of resource the capability provides access to and the
//! contained value is a pair whose first element is a resource ID, usually an index into a resource
//! specific descriptor table in the pertinent kernel subsystem which contains descriptors that
//! indicate specific units of that resource. The second element of the pair is a set of permissions
//! that determine what actions can be performed on that resource by the holder of that capability.
//! These structures each live under a single spin based `RwLock` to allow for concurrent access for
//! the majority of cases which will be validation, a read-only operation, while allowing for
//! exclusive access for the cases that require modification of the capability table, which will be
//! write operations. This lock is not expected to be held for long periods of time and write code
//! should be written to minimize the time the lock is held. Accordingly, spinning is the preferred
//! locking strategy.
//!
//! The keys themselves are 128 bit cryptographically secure random numbers that are generated
//! when the capability is created. This means that a
//! capability in one isolation domain is not valid in another isolation domain, even if the
//! capability key is the same. This is a key part of the security model of charlottek, as it
//! ensures that capabilities cannot be used to access resources in other isolation domains unless
//! explicitly shared by a thread running in the isolation domain that owns the capability.

use spin::RwLock;

use crate::memory::pmem::frame_set::{FrameSet, FrameSetPermissions};

mod isolation_domain;

pub type CapabilityKey = u32;

pub enum CapabilityDescriptor<'resource> {
    FrameSet(&'resource RwLock<FrameSet>, FrameSetPermissions),
    //AddressSpace(&'resource RwLock<AddressSpace>, AddressSpacePermissions),
}
