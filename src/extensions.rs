use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct Extension {

    pub name: String,

    pub path: PathBuf,

    pub enabled: bool,

}



fn extension_folder() -> PathBuf {

    dirs::config_dir()
        .unwrap()
        .join("pae")
        .join("extensions")

}



fn state_file(path: &PathBuf) -> PathBuf {

    path.join("enabled.txt")

}



pub fn load_extensions() -> Vec<Extension> {


    let mut result = Vec::new();


    let folder =
        extension_folder();



    if !folder.exists() {

        return result;

    }



    if let Ok(entries) =
        fs::read_dir(folder)
    {


        for entry in entries.flatten()
        {


            let path =
                entry.path();



            if path.is_dir()
            {


                let name =
                    path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();



                let enabled =

                    if state_file(&path).exists() {

                        let value =
                            fs::read_to_string(
                                state_file(&path)
                            )
                            .unwrap_or_default();


                        value.trim() == "true"


                    } else {

                        false

                    };



                result.push(
                    Extension {

                        name,

                        path,

                        enabled,

                    }
                );

            }

        }

    }



    result

}




pub fn toggle_extension(
    ext: &Extension
) {


    let file =
        state_file(
            &ext.path
        );



    let new_state =
        !ext.enabled;



    fs::write(
        file,
        new_state.to_string()
    )
    .unwrap();



    println!(
        "{} -> {}",
        ext.name,
        if new_state {
            "aktiv"
        } else {
            "deaktiviert"
        }
    );


}