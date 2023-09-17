use crate::{
    proto,
    util::{Id, KeyPair},
};
// enum permission {Write, AskChange, OnlyRead}
// TODO: struct device{
// keypair
// device_id
// trusted
// configurations manifest
// premissions: Vec<(Topic[or subtopic], permission)>
// }
pub enum Permission {
    Write,
    AskChange,
    OnlyRead,
}

pub struct Device {
    keypair: KeyPair,
    device_id: Id,
    trusted: DeviceTrust,
    permissions: Vec<(proto, Permission)>,
    // Syncer: Vec<impl syncer> ex: a device maybe syncable by syncthing or google cloud but not by filecoin
}
/// Devices can be Trusted(ex: your on devices synced by syncthing) or untrusted(ex: cloud seevices like google drive or p2p networks)
pub enum DeviceTrust {
    Untrusted { keep_state: bool },
    Trusted,
}
