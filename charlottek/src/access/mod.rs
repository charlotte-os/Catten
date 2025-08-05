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

mod ephemeral;
mod persistent;
