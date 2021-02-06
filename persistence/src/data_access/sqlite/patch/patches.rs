use crate::data_access::sqlite::patch::list::Patch1;
use crate::data_access::sqlite::patch::list::Patch2;
use crate::data_access::sqlite::patch::patch::Patch;

/// List of available database patches.
pub(crate) const LIST: [&dyn Patch; 2] = [&Patch1 {}, &Patch2 {}];
