mod editor;
mod file_manager;
mod settings_window;
mod tabs;

use gtk4::prelude::*;

use gtk4::{
    Application,
    ApplicationWindow,
    Box,
    Button,
    HeaderBar,
    Notebook,
    Orientation,
    Paned,
};

use tabs::TabManager;



fn main() {

    let app =
        Application::builder()
        .application_id(
            "com.jayden.pae"
        )
        .build();



    app.connect_activate(
        |app| {


        let window =
            ApplicationWindow::builder()
            .application(app)
            .title(
                "PAE Rust Editor 0.5"
            )
            .default_width(
                1200
            )
            .default_height(
                800
            )
            .build();




        let header =
            HeaderBar::new();



        let new_button =
            Button::with_label(
                "Neu"
            );


        let open_button =
            Button::with_label(
                "Öffnen"
            );


        let save_button =
            Button::with_label(
                "Speichern"
            );


        let extensions_button =
            Button::with_label(
                "🧩 Extensions"
            );


        let settings_button =
            Button::with_label(
                "⚙ Einstellungen"
            );




        header.pack_start(
            &new_button
        );


        header.pack_start(
            &open_button
        );


        header.pack_start(
            &save_button
        );



        header.pack_end(
            &settings_button
        );


        header.pack_end(
            &extensions_button
        );



        window.set_titlebar(
            Some(
                &header
            )
        );






        let notebook =
            Notebook::new();




        let manager =
            TabManager::new(
                &notebook
            );



        let editor =
            manager.new_tab(
                "Unbenannt"
            );






        // leere Seitenleiste bleibt für später

        let sidebar =
            Box::new(
                Orientation::Vertical,
                5
            );


        sidebar.set_width_request(
            10
        );






        let paned =
            Paned::new(
                Orientation::Horizontal
            );



        paned.set_start_child(
            Some(
                &sidebar
            )
        );


        paned.set_end_child(
            Some(
                &notebook
            )
        );





        let layout =
            Box::new(
                Orientation::Vertical,
                0
            );



        layout.append(
            &paned
        );



        window.set_child(
            Some(
                &layout
            )
        );







        // Neuer Tab

        let manager_new =
            manager.clone();


        new_button.connect_clicked(
            move |_| {

                manager_new
                .new_tab(
                    "Unbenannt"
                );

            }
        );








        // Einstellungen

        let settings_parent =
            window.clone();


        let settings_editor =
            editor.clone();



        settings_button.connect_clicked(
            move |_| {


                settings_window::show_settings(
                    &settings_parent,
                    &settings_editor
                );


            }
        );








        // Öffnen

        let open_window =
            window.clone();


        let open_notebook =
            notebook.clone();


        let open_editor =
            editor.clone();



        open_button.connect_clicked(
            move |_| {


                file_manager::open_file(
                    &open_window,
                    &open_notebook,
                    &open_editor
                );


            }
        );








        // Speichern

        let save_window =
            window.clone();


        let save_editor =
            editor.clone();



        save_button.connect_clicked(
            move |_| {


                file_manager::save_file(
                    &save_window,
                    &save_editor
                );


            }
        );







        // Extensions Button bleibt,
        // macht momentan nichts


        extensions_button.connect_clicked(
            |_| {

                println!(
                    "Extensions kommen später"
                );

            }
        );







        window.present();


        }

    );



    app.run();

}