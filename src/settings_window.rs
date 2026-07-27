use gtk4::prelude::*;

use gtk4::{
    ApplicationWindow,
    Window,
    Box,
    Button,
    Label,
    Entry,
    ComboBoxText,
    SpinButton,
    CheckButton,
    TextView,
    Orientation,
    CssProvider,
    gdk,
};



pub fn show_settings(
    parent: &ApplicationWindow,
    editor: &TextView,
) {


    let window =
        Window::builder()
        .title("PAE Einstellungen")
        .default_width(450)
        .default_height(500)
        .transient_for(parent)
        .modal(true)
        .build();



    let layout =
        Box::new(
            Orientation::Vertical,
            10
        );


    layout.set_margin_top(20);
    layout.set_margin_bottom(20);
    layout.set_margin_start(20);
    layout.set_margin_end(20);



    let title =
        Label::new(
            Some(
                "Editor Einstellungen"
            )
        );



    let font =
        ComboBoxText::new();


    font.append_text(
        "Monospace"
    );


    font.append_text(
        "Sans"
    );


    font.append_text(
        "Serif"
    );


    font.set_active(
        Some(0)
    );



    let size =
        SpinButton::with_range(
            8.0,
            60.0,
            1.0
        );


    size.set_value(
        14.0
    );



    let text_color =
        Entry::new();


    text_color.set_text(
        "#000000"
    );



    let background =
        Entry::new();


    background.set_text(
        "#ffffff"
    );



    let dark =
        CheckButton::with_label(
            "Dunkler Editor"
        );



    let apply =
        Button::with_label(
            "Übernehmen"
        );



    let editor_clone =
        editor.clone();



    let font_clone =
        font.clone();


    let size_clone =
        size.clone();


    let fg_clone =
        text_color.clone();


    let bg_clone =
        background.clone();


    let dark_clone =
        dark.clone();



    apply.connect_clicked(
        move |_| {


        let family =
            font_clone
            .active_text()
            .unwrap()
            .to_string();



        let font_size =
            size_clone.value();



        let mut fg =
            fg_clone.text()
            .to_string();



        let mut bg =
            bg_clone.text()
            .to_string();



        if dark_clone.is_active()
        {

            fg =
                "#ffffff".to_string();


            bg =
                "#202020".to_string();

        }



        let css =
format!(
r#"

textview {{

font-family: {};

font-size: {}pt;

color: {};

background: {};

caret-color: {};

}}

"#,
            family,
            font_size,
            fg,
            bg,
            fg
);



        let provider =
            CssProvider::new();



        provider.load_from_data(
            &css
        );



        if let Some(display) =
            gdk::Display::default()
        {


            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION
            );


        }



        editor_clone
        .queue_draw();



        println!(
            "Einstellungen angewendet"
        );


        }
    );





    layout.append(
        &title
    );


    layout.append(
        &Label::new(
            Some(
                "Schriftart"
            )
        )
    );


    layout.append(
        &font
    );


    layout.append(
        &Label::new(
            Some(
                "Schriftgröße"
            )
        )
    );


    layout.append(
        &size
    );


    layout.append(
        &Label::new(
            Some(
                "Schriftfarbe"
            )
        )
    );


    layout.append(
        &text_color
    );


    layout.append(
        &Label::new(
            Some(
                "Hintergrund"
            )
        )
    );


    layout.append(
        &background
    );


    layout.append(
        &dark
    );


    layout.append(
        &apply
    );



    window.set_child(
        Some(
            &layout
        )
    );


    window.present();

}