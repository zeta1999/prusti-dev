// © 2019, ETH Zurich
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Fix the potentially broken encoding.

mod ghost_vars;
mod loops;

pub use self::ghost_vars::fix_ghost_vars;
pub use self::loops::havoc_assigned_locals;
