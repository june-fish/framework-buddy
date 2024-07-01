use framework_lib::chromium_ec::{CrosEc, CrosEcDriverType};

pub fn get_cros_ec() -> Option<CrosEc> {
    // CrocEc driver is currently broken on AMD platforms, should be fixed in linux 6.10
    // The Platform enum is private and I don't feel like patching it so enjoy this hack

    println!("{:?}", framework_lib::smbios::get_platform().unwrap());

    let driver = match format!("{:?}", framework_lib::smbios::get_platform().unwrap()).as_str() {
        "Framework13Amd" | "Framework16" => CrosEcDriverType::Portio,
        _ => CrosEcDriverType::CrosEc,
    };

    println!("{:?}", driver);

    CrosEc::with(driver)
}

pub fn get_keyboard_backlight_safe(ec: CrosEc) -> u8 {
    // For some reason if the keyboard backlight is set to 0, this will panic
    // So here's another hack

    std::panic::catch_unwind(|| ec.get_keyboard_backlight().unwrap()).unwrap_or(0)
}
