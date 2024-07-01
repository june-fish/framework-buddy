mod brightness;
mod charge_limit;
mod info;
mod util;

use crate::brightness::Brightness;
use crate::charge_limit::ChargeLimit;
use crate::info::Info;

use gtk::prelude::{GtkWindowExt, OrientableExt};
use libhelium::prelude::*;
use relm4::{
    gtk::{self, gio::ApplicationFlags},
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
    SimpleComponent,
};
use relm4_icons;

const APPID: &str = "fish.june.framework-buddy";

struct AppModel {
    nav: gtk::Stack,
    info: Controller<Info>,
    brightness: Controller<Brightness>,
    charge_limit: Controller<ChargeLimit>,
}

#[derive(Debug)]
enum AppMsg {}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = u8;

    type Input = AppMsg;
    type Output = ();

    view! {
        libhelium::ApplicationWindow {
            set_title: Some("Framework Buddy"),
            set_default_width: 800,
            set_default_height: 600,

            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                libhelium::NavigationRail {
                    set_stack = &model.nav.clone(),
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    libhelium::AppBar {},
                    gtk::Box{
                        #[local_ref]
                        nav_rail_stack_ref -> gtk::Stack {
                            set_hexpand: true,
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
        let nav_rail_stack = gtk::Stack::new();
        let nav_rail_stack_ref = &nav_rail_stack;

        let info: Controller<Info> = Info::builder()
            .launch(0)
            .forward(sender.input_sender(), |_| todo!());
        let info_page = nav_rail_stack.add_child(info.widget());
        info_page.set_icon_name("info-symbolic");
        info_page.set_title("Info");

        let brightness: Controller<Brightness> = Brightness::builder()
            .launch(0)
            .forward(sender.input_sender(), |_| todo!());
        let brightness_page = nav_rail_stack.add_child(brightness.widget());
        brightness_page.set_icon_name("display-brightness-symbolic");
        brightness_page.set_title("Brightness");

        let charge_limit: Controller<ChargeLimit> = ChargeLimit::builder()
            .launch(0)
            .forward(sender.input_sender(), |_| todo!());
        let charge_limit_page = nav_rail_stack.add_child(charge_limit.widget());
        charge_limit_page.set_icon_name("battery-symbolic");
        charge_limit_page.set_title("Charge Limit");

        let model = AppModel {
            nav: nav_rail_stack.clone(),
            info,
            brightness,
            charge_limit,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    relm4_icons::initialize_icons();

    let app = libhelium::Application::builder()
        .application_id(APPID)
        .flags(ApplicationFlags::default())
        .build();
    let app = RelmApp::from_app(app);
    app.run::<AppModel>(0);
}
