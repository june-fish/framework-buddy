use crate::util::get_cros_ec;
use framework_lib::chromium_ec::CrosEc;
use libhelium::prelude::*;
use relm4::gtk::{self, prelude::*};
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

pub struct Info {
    ec: CrosEc,
    privacy_info: (bool, bool),
}

#[derive(Debug)]
pub enum InfoMsg {
    UpdateInfo,
}

#[relm4::component(pub)]
impl SimpleComponent for Info {
    type Init = u8;
    type Input = InfoMsg;
    type Output = ();

    view! {
        &gtk::Box {
            set_spacing: 5,
            set_margin_all: 5,
            set_orientation: gtk::Orientation::Vertical,
            set_halign: gtk::Align::Center,
            libhelium::ContentList {
                set_title: Some("Info"),
                set_description: Some("Status of camera/mic connection."),
                add = &libhelium::ContentBlock {
                    set_title: "Camera",
                    #[watch]
                    set_icon: match model.privacy_info.1 {
                        true => "settings-camera-symbolic",
                        false => "camera-disabled-symbolic",
                    },
                    set_child = &gtk::Label {
                        #[watch]
                        set_label: match model.privacy_info.1 {
                            true => "Connected",
                            false => "Disconnected",
                        },
                    },
                },
                add = &libhelium::ContentBlock {
                    set_title: "Microphone",
                    #[watch]
                    set_icon: match model.privacy_info.0 {
                        true => "settings-microphone-symbolic",
                        false => "microphone-disabled-symbolic",
                    },
                    set_child = &gtk::Label {
                        #[watch]
                        set_label: match model.privacy_info.0 {
                            true => "Connected",
                            false => "Disconnected",
                        },
                    },
                },
            },
        },
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let ec = get_cros_ec().unwrap();

        let model = Info {
            ec: ec.clone(),
            privacy_info: ec.get_privacy_info().unwrap(),
            // privacy_info: (false,false),
        };

        let widgets = view_output!();

        gtk::glib::source::timeout_add_seconds(1, move || {
            sender.input(InfoMsg::UpdateInfo);
            libhelium::glib::ControlFlow::Continue
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            InfoMsg::UpdateInfo => {
                self.privacy_info = self.ec.get_privacy_info().unwrap();
                // self.privacy_info = (false, false);
            }
        }
    }
}
