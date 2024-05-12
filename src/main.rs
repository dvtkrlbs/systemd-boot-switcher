use efivar::{efi::{Variable, VariableVendor}, system};
use uuid::Uuid;
use std::str::FromStr;


const VENDOR: &'static str = "4a67b082-0a4c-41cf-b6c7-440b29bb8c4f";
const DEFAULT_ENTRY_NAME: &'static str = "LoaderEntryDefault";
const CONFIG_TIMEOUT_NAME: &'static str = "LoaderConfigTimeout";
const ENTRY_ONESHOT_NAME: &'static str = "LoaderEntryOneShot";
const DEVICE_IDENTIFIER_NAME: &'static str = "LoaderDeviceIdentifier";
const DEVICE_PART_UUID_NAME : &'static str = "LoaderDevicePartUUID";

fn main() {
    let system_manager = system();

    let variable = Variable::new_with_vendor(&DEVICE_PART_UUID_NAME, VariableVendor::Custom(Uuid::from_str(&VENDOR).unwrap()));

    let value = system_manager.read(&variable).unwrap().0;

    println!("{}", String::from_utf8(value).unwrap());
}
