pub fn add(left: usize, right: usize) -> usize {
    left + right
}
mod device;
mod util;

// TODO: struct account {
// devices: Vec(device)
// Userid
// keypair
// }

// TODO: moved to another mod struct proto{
// name = string
// data = vec<u8>
// }
///
pub struct proto {
    /// path items separated by /
    ///protocol can have multiple sub protocols
    /// ex: rss/feed
    ///     rss/saved
    path: String,
    data: Vec<u8>,
}
// type protos = vec<proto>

// TODO: trait syncer Ex: syncthing, drive, filecoin
// methods:
// config<T>(config: ) ex: protos sub
// proto ex: rss_proto
// notify_changes
// fn send_changes(hash, signature, data) -> ex: if untrusted make envelope encription

// TODO: struct syncchain {
// local_state: || path to dir
// global_state: ||
// }

// fn new_synchain(local_state, syncer)
// fn make_changes(proto,diff of the changes, device keypair, timestamp, shared encription key)
// -> result<(hash, signature)>{
// checks device permission
// timestamp example: let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
// println!("{}", time);
// hash of (diff, timestamp, device_id) obs: is the name of the directory name
// sign the hash
// }

// fn subscribe to new proto obs: func to change syncer config or to change device dir
// mod util ||
// TODO: keypair,
// TODO: id,
// TODO: sign,
// TODO: diff,
// TODO: encript/envelope
// TODO: timestamp

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
