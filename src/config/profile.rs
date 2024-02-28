use hidapi::HidDevice;
use crate::lib::getstatus::check_sleep;

pub fn set(device: &HidDevice, id: u8) {
    check_sleep(device);

    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x01;
    bfr[6] = 0x05;
    bfr[7] = id;

    device.send_feature_report(&bfr).unwrap();
}
