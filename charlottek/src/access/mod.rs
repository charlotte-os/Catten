//! Capability Based Access Control System
//!
//! A capability is an unforgeable token of authority associated with specific units of a system
//! resource along with a set of permissions that determine what actions can be performed on that
//! resource by the holder of that capability. In a capability based access control system, two
//! facts must always hold true:
//! 1. The holder of a capability can only perform actions on the resource associated with that
//!    capability if those actions fall within the permission set associated with that capability.
//! 2. Holding and using a capability associated with a given unit of a resource as well as the
//!    appropriate permissions is the only means by which to perform any given action on that
//!    resource.
//!
//! In `charlottek`, capabilities are keys for descriptors in a global capability table. Each
//! descriptor is an ADT whose variant represents the type of resource the capability provides
//! access to and the contained value is a pair whose first element is an index into a resource
//! specific descriptor table which contains descriptors that indicate specific units of that
//! resource. The second element of the pair is a set of permissions that determine what actions can
//! be performed on that resource by the holder of that capability. These structures all live under
//! a single spin based `RwLock` to allow for concurrent access for the majority of cases which will
//! be validation, a read-only operation, while allowing for exclusive access for the cases that
//! require modification of the capability table, which will be write operations. This lock is not
//! expected to be held for long periods of time and write code should be written to minimize the
//! time the lock is held. Accordingly, spinning is the preferred locking strategy.
//!
//! The keys themselves are 128 bit cryptographically secure random numbers that are generated
//! when the capability is created. This provides an absurdly large key space that is
//! practically impossible to brute force or have a collision in and each process context has its
//! own key space so that capabilities are not shared between processes. This means that a
//! capability in one process context is not valid in another process context, even if the
//! capability is the same. This is a key part of the security model of charlottek, as it
//! ensures that capabilities cannot be used to access resources in other process contexts unless
//! explicitly shared by the process that owns the capability.

use core::mem::MaybeUninit;

use hashbrown::HashMap;
use spin::RwLock;

pub enum Error {
    KeyNotFound,
    KeyExpired,
    CapabilityTypeMismatch,
}

type CapabilityKey = u64;

static ACCESS_CONTROL_TABLE: RwLock<MaybeUninit<HashMap<CapabilityKey, AccessDescriptor>>> =
    RwLock::new(MaybeUninit::uninit());

pub enum AccessDescriptor {
    PageFrameSet(FrameSetId, PageFramePerms),
    AddressSpace(AddressSpaceId, AddressSpacePerms),
    Thread(ThreadId, ThreadPerms),
}
