use crate::util::{get_cros_ec, get_keyboard_backlight_safe};
use framework_lib::chromium_ec::{commands::FpLedBrightnessLevel, CrosEc};
use libhelium::prelude::*;
use num_traits::cast::FromPrimitive;
use relm4::gtk::{self, prelude::*};
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

pub struct Brightness {
    ec: CrosEc,
    keyboard_brightness: u8,
    fp_brightness: FpLedBrightnessLevel,
}

#[derive(Debug)]
pub enum BrightnessMsg {
    SetFpBrightness(FpLedBrightnessLevel),
    SetKeyboardBrightness(u8),
    UpdateInfo,
}

#[relm4::component(pub)]
impl SimpleComponent for Brightness {
    type Init = u8;
    type Input = BrightnessMsg;
    type Output = ();

    view! {
    &gtk::Box {
        set_spacing: 5,
        set_margin_all: 5,
        set_orientation: gtk::Orientation::Vertical,
        set_halign: gtk::Align::Center,
        libhelium::SettingsList {
            set_title: Some("Brightness"),
            add = &libhelium::SettingsRow {
                set_title: "Fingerprint Sensor",
                set_subtitle: "Set the fingerprint sensor brightness",
                set_icon: "auth-fingerprint-symbolic",
                add = &gtk::DropDown::from_strings(&[&"High", &"Medium", &"Low"]) {
                    connect_selected_item_notify[sender] => move |value| {
                        sender.input(match value.selected() {
                            0 => BrightnessMsg::SetFpBrightness(FpLedBrightnessLevel::High),
                            1 => BrightnessMsg::SetFpBrightness(FpLedBrightnessLevel::Medium),
                            2 => BrightnessMsg::SetFpBrightness(FpLedBrightnessLevel::Low),
                            _ => todo!(),
                        })
                    },
                    #[watch]
                    set_selected: match model.fp_brightness {
                        FpLedBrightnessLevel::High => 0,
                        FpLedBrightnessLevel::Medium => 1,
                        FpLedBrightnessLevel::Low => 2,
                    }
                },
            },
            add = &libhelium::SettingsRow {
                set_title: "Keyboard",
                set_subtitle: "Set the keyboard brightness",
                set_icon: "settings-keyboard-symbolic",

                add = &gtk::Scale {
                    #[watch]
                    set_value: model.keyboard_brightness as f64,
                    set_digits: 0,
                    set_draw_value: true,
                    set_value_pos: gtk::PositionType::Left,
                    set_has_origin: true,
                    set_range: (0.0, 100.0),
                    connect_value_changed[sender] => move |kbvalue| {
                        println!("{:?}", kbvalue.value());
                        sender.input(BrightnessMsg::SetKeyboardBrightness(kbvalue.value() as u8))
                    }
                }
            },
        },
        } -> {
            set_title: "Brightness",
            set_icon_name: "display-brightness-symbolic",
        },
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let ec = get_cros_ec().unwrap();

        let model = Brightness {
            ec: ec.clone(),
            keyboard_brightness: get_keyboard_backlight_safe(ec.clone()),
            fp_brightness: match ec.get_fp_led_level().unwrap() {
                15 => FpLedBrightnessLevel::Low,
                40 => FpLedBrightnessLevel::Medium,
                55 => FpLedBrightnessLevel::High,
                _ => todo!(),
            },
            // keyboard_brightness: 80,
            // fp_brightness: FpLedBrightnessLevel::Low,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            BrightnessMsg::SetFpBrightness(level) => {
                let level_u8 = (level as u8).clone();
                self.fp_brightness = FpLedBrightnessLevel::from_u8(level_u8.clone()).unwrap();
                self.ec
                    .set_fp_led_level(FpLedBrightnessLevel::from_u8(level_u8.clone()).unwrap())
                    .unwrap();
            }
            BrightnessMsg::SetKeyboardBrightness(level) => {
                self.keyboard_brightness = level as u8;
                self.ec.set_keyboard_backlight(level);
            }
            BrightnessMsg::UpdateInfo => {
                self.keyboard_brightness = get_keyboard_backlight_safe(self.ec.clone());
                self.fp_brightness = match self.ec.get_fp_led_level().unwrap() {
                    15 => FpLedBrightnessLevel::Low,
                    40 => FpLedBrightnessLevel::Medium,
                    55 => FpLedBrightnessLevel::High,
                    _ => todo!(),
                }
            }
        }
    }
}
