use std::io;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    get_volume_info(&hklm);
    get_volume_guid_driveletter(&hklm);
    //open_registry_tree(&cur_ver)?;
    Ok(())
}

fn get_volume_info(registry: &RegKey) {
    let cur_ver = registry
        .open_subkey("Software\\Microsoft\\Windows Portable Devices\\Devices")
        .unwrap();
    for i in cur_ver.enum_keys().map(|x| x.unwrap()) {
        let devices = i.split("#").collect::<Vec<&str>>();
        if devices.len() == 6 {
            println!("> USB Serial Number is : {:?}", devices[4]);
            let friendly_name: String = cur_ver
                .open_subkey(i)
                .unwrap()
                .get_value("FriendlyName")
                .unwrap();
            println!(">> Friendly name : {:?}", friendly_name);
        }
    }
}

fn get_volume_guid_driveletter(registry: &RegKey) {
    let cur_ver = registry.open_subkey("System\\MountedDevices").unwrap();
    for i in cur_ver.enum_values().map(|x| x.unwrap()) {
        //println!("{}", i.0);
        let array: Vec<u8> = i.1.bytes;
        if vec_contain_non_ascii(&array) {
            continue;
        }
        println!("{}", std::str::from_utf8(&array).unwrap());
    }
}

fn open_registry_tree(registry: &RegKey) -> io::Result<()> {
    for i in registry.enum_keys().map(|x| x.unwrap()) {
        println!("{:?}", i);
        let registry_sub = match registry.open_subkey(i) {
            Ok(reg) => reg,
            Err(ref e) if e.kind() == io::ErrorKind::PermissionDenied => {
                println!("ACCESS DENIED");
                println!("");
                break;
            }
            Err(_) => panic!("Panic"),
        };
        let info = registry_sub.query_info()?;
        println!("{:?}", info.sub_keys);
        open_registry_tree(&registry_sub).unwrap();
    }
    Ok(())
}

fn vec_contain_non_ascii(array: &Vec<u8>) -> bool {
    for i in array {
        if !i.is_ascii() {
            return true;
        }
    }
    false
}
