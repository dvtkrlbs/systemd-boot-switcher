use efivar::{efi::{Variable, VariableFlags, VariableVendor}, system};
use uuid::Uuid;
use windows_drives::{BufferedHarddiskVolume, HarddiskVolume, PhysicalDrive};
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
use std::{ffi::CString, fs::File, io::{Read, Seek, Write, WriterPanicked}, str::FromStr};


const VENDOR: &'static str = "4a67b082-0a4c-41cf-b6c7-440b29bb8c4f";
const DEFAULT_ENTRY_NAME: &'static str = "LoaderEntryDefault";
const CONFIG_TIMEOUT_NAME: &'static str = "LoaderConfigTimeout";
const ENTRY_ONESHOT_NAME: &'static str = "LoaderEntryOneShot";
const DEVICE_IDENTIFIER_NAME: &'static str = "LoaderDeviceIdentifier";
const DEVICE_PART_UUID_NAME : &'static str = "LoaderDevicePartUUID";
const LOADER_FEATURES_NAME: &'static str = "LoaderFeatures";


struct WriteNoOpBufferenHarddiskVolume {
    inner: BufferedHarddiskVolume,
}

impl Read for WriteNoOpBufferenHarddiskVolume {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Seek for WriteNoOpBufferenHarddiskVolume {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.inner.seek(pos)
    }
}

impl Write for WriteNoOpBufferenHarddiskVolume {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl WriteNoOpBufferenHarddiskVolume {
    fn new(volume: BufferedHarddiskVolume) -> Self {
        Self {
            inner: volume
        }
    }
}

fn main() {
    let mut system_manager = system();

    let variable = Variable::new_with_vendor(&DEFAULT_ENTRY_NAME, VariableVendor::Custom(Uuid::from_str(&VENDOR).unwrap()));
    let value = system_manager.read(&variable).unwrap().0;

    println!("{}", String::from_utf8(value).unwrap());

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let setup = hklm.open_subkey("SYSTEM\\Setup").unwrap();
    let system_partition: String = setup.get_value("SystemPartition").unwrap();



    let volume = BufferedHarddiskVolume::open(1).unwrap();

    // dbg!(&volume);


    // let boot_volume = File::open(partition_name).unwrap();

    let fs = fatfs::FileSystem::new(WriteNoOpBufferenHarddiskVolume::new(volume), fatfs::FsOptions::new()).unwrap();
// 
    let mut efi_dir = fs.root_dir().open_dir("loader").unwrap().open_file("loader.conf").unwrap();

    let mut content = String::new();

    efi_dir.read_to_string(&mut content);
    // println!("{}", content);
    fs.root_dir().open_dir("loader").unwrap().iter().for_each(|f| { 
        let file = f.unwrap();
        // println!("{}", file.file_name());
    });
    // println!("{}", content);


    let oneshot_variable = Variable::new_with_vendor(&ENTRY_ONESHOT_NAME, VariableVendor::Custom(Uuid::from_str(&VENDOR).unwrap()));

    // let val = "nixos-generation-71.conf";
    let val = "nixos-generation-71";
    let mut buffer = Vec::with_capacity(val.len() * 2 + 2);

    for wchar in val.encode_utf16() {
        let [first, second] = wchar.to_le_bytes();
        buffer.push(first);
        buffer.push(second);
    }

    buffer.extend_from_slice(&[0, 0]);
    match system_manager.write(&oneshot_variable, VariableFlags::NON_VOLATILE | VariableFlags::BOOTSERVICE_ACCESS | VariableFlags::RUNTIME_ACCESS, &buffer) {
        Ok(()) => println!("SUCCESFULLY SETO ONESHOT"),
        Err(e) => println!("{}", e.to_string())
    }

    let loader_features = Variable::new_with_vendor(&LOADER_FEATURES_NAME, VariableVendor::Custom(Uuid::from_str(&VENDOR).unwrap()));

    // match system_manager.write(&loader_features, VariableFlags::empty(), &[1 << 0 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4]) {
        // Ok(()) => println!("SUCCESFULLY SETO ONESHOT"),
        // Err(e) => println!("{}", e.to_string())
    // }



    let res = system_manager.read(&oneshot_variable).unwrap();
    println!("{}", String::from_utf8(res.0).unwrap());

    // let res = system_manager.read(&loader_features).unwrap();
    // println!("{}", res.0[0]);



    // let volume = HarddiskVolume::open(1).unwrap();
}
