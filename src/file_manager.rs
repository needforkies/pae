use gtk4::prelude::*;

use gtk4::{
    ApplicationWindow,
    FileChooserAction,
    FileChooserDialog,
    FileFilter,
    ResponseType,
    Notebook,
    TextView,
};



pub fn open_file(
    window: &ApplicationWindow,
    _notebook: &Notebook,
    editor: &TextView,
) {


    let dialog =
        FileChooserDialog::new(
            Some("Datei öffnen"),
            Some(window),
            FileChooserAction::Open,
            &[
                ("Abbrechen", ResponseType::Cancel),
                ("Öffnen", ResponseType::Accept),
            ],
        );



    let filter =
        FileFilter::new();

    filter.add_pattern("*.txt");
    filter.add_pattern("*.rs");

    dialog.add_filter(
        &filter
    );



    let editor_clone =
        editor.clone();



    dialog.connect_response(
        move |dialog, response| {


            if response == ResponseType::Accept {


                if let Some(file) =
                    dialog.file()
                {


                    if let Some(path) =
                        file.path()
                    {


                        match std::fs::read_to_string(
                            &path
                        ) {


                            Ok(text) => {

                                editor_clone
                                    .buffer()
                                    .set_text(
                                        &text
                                    );

                                println!(
                                    "Geöffnet: {:?}",
                                    path
                                );

                            }



                            Err(e) => {

                                println!(
                                    "Fehler: {}",
                                    e
                                );

                            }

                        }


                    }

                }

            }


            dialog.close();

        }
    );



    dialog.show();

}





pub fn save_file(
    window: &ApplicationWindow,
    editor: &TextView,
) {


    let dialog =
        FileChooserDialog::new(
            Some("Datei speichern"),
            Some(window),
            FileChooserAction::Save,
            &[
                ("Abbrechen", ResponseType::Cancel),
                ("Speichern", ResponseType::Accept),
            ],
        );



    let editor_clone =
        editor.clone();



    dialog.connect_response(
        move |dialog, response| {


            if response == ResponseType::Accept {


                if let Some(file) =
                    dialog.file()
                {


                    if let Some(path) =
                        file.path()
                    {


                        let buffer =
                            editor_clone.buffer();



                        let text =
                            buffer.text(
                                &buffer.start_iter(),
                                &buffer.end_iter(),
                                false
                            );



                        std::fs::write(
                            path,
                            text.as_str()
                        )
                        .unwrap();



                        println!(
                            "Gespeichert"
                        );

                    }

                }

            }


            dialog.close();

        }
    );



    dialog.show();

}