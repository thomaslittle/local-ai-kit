// TEMPORARY diagnostic; removed before merge.
#[cfg(unix)]
#[test]
fn probe_unix_socket_bind() {
    use interprocess::local_socket::{GenericFilePath, ListenerOptions, prelude::*};
    let p = std::env::temp_dir().join("lai-probe-1.sock");
    eprintln!("PROBE path={} len={}", p.display(), p.as_os_str().len());
    match std::os::unix::net::UnixListener::bind(&p) {
        Ok(_) => eprintln!("PROBE std bind ok"),
        Err(e) => eprintln!("PROBE std bind err {e:?}"),
    }
    let _ = std::fs::remove_file(&p);
    let name = p.to_str().unwrap().to_fs_name::<GenericFilePath>().unwrap();
    match ListenerOptions::new().name(name).create_sync() {
        Ok(_) => eprintln!("PROBE interprocess ok"),
        Err(e) => eprintln!("PROBE interprocess err {e:?}"),
    }
    let _ = std::fs::remove_file(&p);
    let short = std::path::PathBuf::from("/tmp/lai-probe-2.sock");
    let name = short
        .to_str()
        .unwrap()
        .to_fs_name::<GenericFilePath>()
        .unwrap();
    match ListenerOptions::new().name(name).create_sync() {
        Ok(_) => eprintln!("PROBE interprocess /tmp ok"),
        Err(e) => eprintln!("PROBE interprocess /tmp err {e:?}"),
    }
}
