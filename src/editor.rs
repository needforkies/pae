use gtk4::prelude::*;

use gtk4::{
    Box,
    Orientation,
    ScrolledWindow,
    TextView,
};


#[derive(Clone)]
pub struct Editor {

    pub container: Box,

    pub text_view: TextView,

}


impl Editor {


    pub fn new(name: &str) -> Self {


        let text_view =
            TextView::new();


        text_view.set_monospace(true);


        text_view.set_wrap_mode(
            gtk4::WrapMode::None
        );


        let scroll =
            ScrolledWindow::builder()
                .child(&text_view)
                .hexpand(true)
                .vexpand(true)
                .build();



        let container =
            Box::new(
                Orientation::Vertical,
                0
            );


        container.append(
            &scroll
        );


        Self {

            container,

            text_view,

        }

    }



    pub fn clear(&self) {

        self.text_view
            .buffer()
            .set_text("");

    }


}