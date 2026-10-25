//! 站点展示配置。
//!
//! 修改后重新构建即可生效。

/// 是否在「关于 → 硬件」中显示购入金额。
///
/// 设为 `false` 后，价格列、金额说明和相关布局会一起隐藏。
pub const SHOW_HARDWARE_PRICES: bool = false;

/// 是否显示「关于 → 硬件」整个分区。
///
/// 设为 `false` 后，编号 03 的硬件区块整体不渲染。
pub const SHOW_HARDWARE_SECTION: bool = true;

/// 要隐藏的硬件分组 key。
///
/// 可用 key：
/// `core`（核心平台）、`storage`（存储与内存）、
/// `peripherals`（外设与机电）、`fees`（其他费用）
///
/// 例如隐藏整个「外设与机电」：`&["peripherals"]`。
pub const HIDDEN_HARDWARE_GROUPS: &[&str] = &["peripherals", "fees"];

/// 要单独隐藏的硬件项 key。
///
/// 可用 key：
/// `cpu`、`mainboard`、`gpu-rx460`、`gpu-gtx1060`、
/// `ssd-kp230-1`、`ram-samsung`、
/// `monitor`、`psu`、`cooler`、`case`、
/// `shipping`、`bios`、`ethernet`
///
/// 例如只隐藏 RX460：`&["gpu-rx460"]`。
pub const HIDDEN_HARDWARE: &[&str] = &["gpu-rx460", "mainboard"];
