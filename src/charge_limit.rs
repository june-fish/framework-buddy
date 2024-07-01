use crate::util::get_cros_ec;
use framework_lib::chromium_ec::CrosEc;
use libhelium::prelude::*;
use relm4::gtk::{self, prelude::*};
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

pub struct ChargeLimit {
    ec: CrosEc,
    max_charge_limit: u8,
    min_charge_limit: u8,
}

#[derive(Debug)]
pub enum ChargeLimitMsg {
    SetMinChargeLimit(u8),
    SetMaxChargeLimit(u8),
    UpdateInfo,
}

#[relm4::component(pub)]
impl SimpleComponent for ChargeLimit {
    type Init = u8;
    type Input = ChargeLimitMsg;
    type Output = ();

    view! {
    &gtk::Box {
        set_spacing: 5,
        set_margin_all: 5,
        set_orientation: gtk::Orientation::Vertical,
        set_halign: gtk::Align::Center,
        libhelium::SettingsList {
            set_title: Some("Charge Limit"),
            add = &libhelium::SettingsRow {
                set_title: "Maximum Charge Limit",
                set_subtitle: "Set the maximum charge limit",
                set_icon: "battery-full-symbolic",
                // #[wrap(Some)]
                // set_child = &gtk::Label {
                //     #[watch]
                //     set_label: &model.max_charge_limit.to_string() as &str,
                // },
                add = &gtk::Scale {
                    #[watch]
                    set_value: model.max_charge_limit as f64,
                    // set_value: 27 as f64,
                    set_digits: 0,
                    set_draw_value: true,
                    set_value_pos: gtk::PositionType::Left,
                    set_has_origin: true,
                    set_range: (20.0, 100.0),
                    connect_value_changed[sender] => move |value| {
                        println!("{:?}", value.value());
                        sender.input(ChargeLimitMsg::SetMaxChargeLimit(value.value() as u8))
                    }
                },
            },
            add = &libhelium::SettingsRow {
                set_title: "Minimum Charge Limit",
                set_subtitle: "Set the minimum charge limit",
                set_icon: "battery-low-symbolic",

                add = &gtk::Scale {
                    #[watch]
                    set_value: model.min_charge_limit as f64,
                    set_digits: 0,
                    set_draw_value: true,
                    set_value_pos: gtk::PositionType::Left,
                    set_has_origin: true,
                    set_range: (0.0, 100.0),
                    connect_value_changed[sender] => move |value| {
                        sender.input(ChargeLimitMsg::SetMinChargeLimit(value.value() as u8))
                    }
                }
            },
        },
        } -> {
            set_title: "Charge Limit",
            set_icon_name: "battery-full-symbolic",
        },
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let ec = get_cros_ec().unwrap();

        let (min_charge_limit, max_charge_limit) = ec.get_charge_limit().unwrap();
        // let (min_charge_limit, max_charge_limit) = (0, 80);

        println!("max: {}% min: {}%", max_charge_limit, min_charge_limit);

        let model = ChargeLimit {
            ec: ec.clone(),
            max_charge_limit,
            min_charge_limit,
        };

        println!("meowy {}", model.max_charge_limit as f64);

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ChargeLimitMsg::SetMaxChargeLimit(limit) => {
                let (curr_min, _) = self.ec.get_charge_limit().unwrap();
                self.max_charge_limit = limit;
                self.ec.set_charge_limit(curr_min, limit).unwrap();
            }
            ChargeLimitMsg::SetMinChargeLimit(limit) => {
                let (_, curr_max) = self.ec.get_charge_limit().unwrap();
                self.min_charge_limit = limit;
                self.ec.set_charge_limit(limit, curr_max).unwrap();
            }
            ChargeLimitMsg::UpdateInfo => {
                let (curr_min, curr_max) = self.ec.get_charge_limit().unwrap();
                self.max_charge_limit = curr_max;
                self.min_charge_limit = curr_min;
            }
        }
    }
}
