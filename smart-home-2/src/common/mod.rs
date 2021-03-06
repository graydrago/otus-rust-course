mod device;
mod report;
mod switch_status_enum;

pub const PRINT_OFFSET: &str = "    ";

pub use device::Device;
pub use report::{report, Report};
pub use switch_status_enum::SwitchStatusEnum;
