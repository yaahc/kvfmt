use kvfmt::kvfmt;

fn main() {
    let dir = "/var/log";
    let paths = vec!["dmesg", "syslog"];

    assert_eq!(
        "dir=/var/log paths=[\"dmesg\", \"syslog\"]",
        kvfmt!(dir, ?paths)
    );
}
