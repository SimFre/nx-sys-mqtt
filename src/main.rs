#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;
use alloc::string::String;

extern crate nx;
use core::panic;
use nx::diag::abort;
use nx::diag::log::lm::LmLogger;
use nx::fs;
use nx::result::*;
use nx::svc;
use nx::util;

use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::error::Error;
use std::time::Duration;
use tokio::{task, time};

#[no_mangle]
pub fn initialize_heap(hbl_heap: util::PointerAndSize) -> util::PointerAndSize {
    if hbl_heap.is_valid() {
        hbl_heap
    } else {
        let heap_size: usize = 0x10000000;
        let heap_address = svc::set_heap_size(heap_size).unwrap();
        util::PointerAndSize::new(heap_address, heap_size)
    }
}

#[no_mangle]
pub fn main() -> Result<()> {
    // Initializing this is not mandatory, but it's helpful for fs to automatically mount the SD by itself
    // fs::initialize_fspsrv_session()?;
    // fs::mount_sd_card("sdmc")?;

    // Example to write file magic to log file.
    // let mut hbmenu_nro =
    //     fs::open_file(String::from("sdmc:/hbmenu.nro"), fs::FileOpenOption::Read())?;
    // hbmenu_nro.seek(0x10, fs::Whence::Current)?; // Skip NRO start (https://switchbrew.org/wiki/NRO)
    // let nro_magic: u32 = hbmenu_nro.read_val()?;

    // let nro_magic_msg = format!("hbmenu NRO magic: {:#X}", nro_magic);
    // let mut log_file = fs::open_file(
    //     String::from("sdmc:/fs-test-log.log"),
    //     fs::FileOpenOption::Create() | fs::FileOpenOption::Write() | fs::FileOpenOption::Append(),
    // )?;
    // log_file.write_array(nro_magic_msg.as_bytes())?;

    // fs::finalize_fspsrv_session();
    // fs::unmount_all();

    // Initialize mqtt
    let mut mqttoptions = MqttOptions::new("tbdClientIdentifier", "192.168.5.43", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    mqttoptions.set_credentials("nxuser", "nxsecret");
    client
        .publish("testtopic", QoS::AtLeastOnce, false, vec![0; 0 as usize])
        .unwrap();
    thread::sleep(Duration::from_millis(100));
    // let (mut client, mut connection) = Client::new(mqttoptions, 10);
    // client.publish("nsw", QoS::AtLeastOnce, false, vec![i; i as usize]).unwrap();

    Ok(())
}

#[panic_handler]
fn panic_handler(info: &panic::PanicInfo) -> ! {
    util::simple_panic_handler::<LmLogger>(info, abort::AbortLevel::FatalThrow())
}
