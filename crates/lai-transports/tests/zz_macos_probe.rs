// TEMPORARY diagnostic; removed before merge.
#[cfg(unix)]
#[tokio::test]
async fn probe_unix_socket_bind() {
    use interprocess::local_socket::{GenericFilePath, ListenerOptions, prelude::*};
    use interprocess::os::unix::local_socket::ListenerOptionsExt;
    let dir = std::env::temp_dir();
    let mk = |n: &str| dir.join(n);
    let p = mk("lai-probe-mode.sock");
    let _ = std::fs::remove_file(&p);
    let name = p.to_str().unwrap().to_fs_name::<GenericFilePath>().unwrap();
    match ListenerOptions::new().name(name).mode(0o600).create_sync() {
        Ok(_) => eprintln!("PROBE sync+mode ok"),
        Err(e) => eprintln!("PROBE sync+mode err {e:?}"),
    }
    let p = mk("lai-probe-tokio.sock");
    let _ = std::fs::remove_file(&p);
    let name = p.to_str().unwrap().to_fs_name::<GenericFilePath>().unwrap();
    match ListenerOptions::new().name(name).create_tokio() {
        Ok(_) => eprintln!("PROBE tokio ok"),
        Err(e) => eprintln!("PROBE tokio err {e:?}"),
    }
    let p = mk("lai-probe-both.sock");
    let _ = std::fs::remove_file(&p);
    let name = p.to_str().unwrap().to_fs_name::<GenericFilePath>().unwrap();
    match ListenerOptions::new().name(name).mode(0o600).create_tokio() {
        Ok(_) => eprintln!("PROBE tokio+mode ok"),
        Err(e) => eprintln!("PROBE tokio+mode err {e:?}"),
    }
}
