use crate::util::status;
use hidapi::HidDevice;

pub fn set(device: &HidDevice, mm: String) -> Result<(), anyhow::Error> {
    status::check_sleep(device)?;

    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x01;
    bfr[5] = 0x01;
    bfr[6] = 0x07;

    bfr[7] = mm.parse::<u8>().unwrap() - 1;

    device.send_feature_report(&bfr)?;

    Ok(())
}
