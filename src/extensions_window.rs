use gtk4::prelude::*;

use gtk4::{
    ApplicationWindow,
    Window,
    Box,
    Button,
    Label,
    Orientation,
};

use crate::extensions::{
    load_extensions,
    toggle_extension,
};



pub fn show_extensions(
    parent: &ApplicationWindow
) {


    let window =
        Window::builder()
        .title(
            "PAE Extensions"
        )
        .default_width(
            600
        )
        .default_height(
            450
        )
        .transient_for(
            parent
        )
        .modal(
            true
        )
        .build();



    let layout =
        Box::new(
            Orientation::Vertical,
            10
        );



    layout.set_margin_top(
        20
    );

    layout.set_margin_bottom(
        20
    );

    layout.set_margin_start(
        20
    );

    layout.set_margin_end(
        20
    );



    let title =
        Label::new(
            Some(
                "🧩 PAE Extension Manager"
            )
        );



    layout.append(
        &title
    );



    let extensions =
        load_extensions();



    if extensions.is_empty()
    {


        let empty =
            Label::new(
                Some(
                    "Keine Extensions installiert"
                )
            );


        layout.append(
            &empty
        );


    }



    for ext in extensions
    {


        let row =
            Box::new(
                Orientation::Horizontal,
                10
            );



        let name =
            Label::new(
                Some(
                    &ext.name
                )
            );



        let status =
            if ext.enabled {

                "Aktiv"

            } else {

                "Deaktiviert"

            };



        let state =
            Label::new(
                Some(
                    status
                )
            );



        let button_text =
            if ext.enabled {

                "Deaktivieren"

            } else {

                "Aktivieren"

            };



        let button =
            Button::with_label(
                button_text
            );



        let extension =
            ext.clone();



        button.connect_clicked(
            move |_| {


                toggle_extension(
                    &extension
                );


            }
        );



        row.append(
            &name
        );


        row.append(
            &state
        );


        row.append(
            &button
        );



        layout.append(
            &row
        );


    }





    let close =
        Button::with_label(
            "Schließen"
        );



    let win_close =
        window.clone();



    close.connect_clicked(
        move |_| {

            win_close.close();

        }
    );



    layout.append(
        &close
    );



    window.set_child(
        Some(
            &layout
        )
    );



    window.present();

}