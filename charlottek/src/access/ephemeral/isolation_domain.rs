//! # Isolation Domain
//!
//! This module is responsible for managing isolation domains within the system. An isolation
//! domain is a logical separation between different execution contexts, ensuring that resources
//! and capabilities are not shared between them unless explicitly allowed. This is a key aspect
//! of the security model in `charlottek`, as it prevents unauthorized access to resources and
//! capabilities across different domains.

use hashbrown::HashMap;

use crate::access::ephemeral::{self, CapabilityDescriptor};
use crate::access::persistent::user::UserIndicator;

type Id = u64;

pub struct IsolationDomain<'a> {
    id: Id,
    owner: UserIndicator, // multiple owners can be achieved through the use of user tags
    capabilities: HashMap<ephemeral::CapabilityKey, CapabilityDescriptor<'a>>,
}

pub struct IsolationTag<'a> {
    id: Id,
    owner: UserIndicator,
    capabilities: HashMap<ephemeral::CapabilityKey, CapabilityDescriptor<'a>>,
}
