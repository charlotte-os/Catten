//! # User Management
//!
//! This module is responsible for managing user accounts and their associated persistent
//! capabilities within the system. It provides functionality for creating, deleting, and modifying
//! user accounts, as well as managing the capabilities associated with each user. The user
//! management system is designed to be flexible and extensible, allowing for the addition of new
//! user tags and persistent capabilities as needed.

use alloc::string::String;

use hashbrown::HashSet;

use crate::access::persistent;

pub type UserId = u64;
pub type UserTagId = u64;

/// This is used so that functions that accept a user can also accept a user tag in a type safe way.
pub enum UserIndicator {
    User(UserId),
    Tag(UserTagId),
}

/// A user is an actual person who uses the system. Unlike in Unix systems where users can also be
/// used to represent other things such as services, the system, and the superuser, in charlottek a
/// user is always expected to be a real person. Users have a unique ID, a name, a full name (first,
/// middle, last), a set of tags, and a set of persistent capabilities.
pub struct User {
    id: UserId,
    name: String,
    full_name: [String; 3],
    tags: HashSet<UserId>,
    capabilities: HashSet<persistent::CapabilityKey>,
}

/// A user tag represents a label or category that can be assigned to a user, allowing for easier
/// management and organization of users within the system. User tags can be used to group users
/// based on shared characteristics, roles, or permissions. User tag IDs exist in a separate ID
/// space from user IDs, but many functions will take either a user ID or a user tag ID through
/// the use of the `UserIndicator` enum.
pub struct UserTag {
    id: UserTagId,
    name: String,
    capabilities: HashSet<persistent::CapabilityKey>,
}
