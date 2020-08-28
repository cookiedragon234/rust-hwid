// rust-hwid
// (c) 2020 tilda, under MIT license
#[cfg(target_os = "windows")]
extern crate winreg;
#[cfg(target_os = "windows")]
use winreg::enums::HKEY_LOCAL_MACHINE;

#[cfg(target_os = "windows")]
pub fn get_id() -> std::string::String {
    // escaping is fun, right? right???
    let hive = winreg::RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("\\\\SOFTWARE\\Microsoft\\Cryptography").expect("Failed to open subkey");
    let id = hive.get_value("MachineGuid").expect("Failed to get MachineGuid");
    return id
}

#[cfg(target_os = "darwin")]
pub fn get_id() -> std::string::String {
    let cmd = std::process::Command::new("ioreg").arg("-rd1").arg("-c").arg("IOExpertPlatformDevice").output().expect("Failed to get HWID");
    let id = cmd.stdout.last();
    return id
}

#[cfg(target_os = "linux")]
pub fn get_id() -> std::string::String {
    let mut id = Path::new("/var/lib/dbus/machine-id").exists();
    if id == false {
        id = Path::new("/etc/machine-id").exists();
        if id == false {
            panic!("No HWID was found on system");
        }
        return id
    }
    return id
}

#[cfg(target_os = "freebsd")]
#[cfg(target_os = "dragonfly")]
#[cfg(target_os = "openbsd")]
#[cfg(target_os = "netbsd")]
pub fn get_id() -> std::string::String {
    unimplemented!("*BSD support is not implemented")
}
