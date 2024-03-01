use crate::args::DPIFn;

pub fn set(bfr: &mut [u8], dpi_fn: DPIFn) {
    bfr[0] = 0x07;
    bfr[1] = 0x01;
    bfr[2] = match dpi_fn {
        DPIFn::StageUp => 1,
        DPIFn::StageDown => 2,
        DPIFn::CycleUp => 6,
        DPIFn::CycleDown => 7,
    };
}
