use serde_derive::{Deserialize, Serialize};
use sewup::types::Raw;
use sewup_derive::{SizedString, Table};

// Table derive provides the handers for CRUD,
// to communicate with these handler, you will need protocol.
// The protocol is easy to build by the `{struct_name}::protocol`, `{struct_name}::Protocol`,
// please check out the test case in the end of this document
#[derive(Table, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub trusted: bool,
    pub age: u8,
}

#[derive(Table, Default, Clone, PartialEq, Serialize, Deserialize)]
#[belongs_to(Person)]
pub struct Post {
    pub content: SizedString!(50),

    // Currently, this field need to set up manually, this will be enhance later
    pub person_id: usize,
}

#[derive(Table, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub address: Raw,
}
