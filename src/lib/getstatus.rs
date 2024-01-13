use colored::Colorize;
use hidapi::HidDevice;
use std::{thread, time::Duration};

pub fn get_bfr_r(device: &HidDevice) -> [u8;65] {
    let mut bfr_w = [0u8; 65];

    bfr_w[3] = 0x02;
    bfr_w[4] = 0x02;
    bfr_w[6] = 0x83;

    device.send_feature_report(&bfr_w).unwrap();

    thread::sleep(Duration::from_millis(50));

    let mut bfr_r = [0u8; 65];

    device.get_feature_report(&mut bfr_r).unwrap();

    bfr_r
}

pub fn get_status(device: &HidDevice) -> usize {
    let mut bfr_r = get_bfr_r(device);

    device.get_feature_report(&mut bfr_r).unwrap();

    let mut status = [0xA1, 0xA4, 0xA2, 0xA0, 0xA3]
        .iter()
        .position(|&s| s == bfr_r[1])
        .unwrap();

    if bfr_r[6] != 0x83 {
        status = 2;
    }

    status
}

pub fn check_sleep(device: &HidDevice) {
    if get_status(device) == 1 {
        println!("Cannot write changes to device since it is off or sleeping.");
    }
}
