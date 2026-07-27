use gtk4::prelude::*;

use gtk4::{
    Label,
    Notebook,
    TextView,
};

use crate::editor::Editor;



#[derive(Clone)]
pub struct TabManager {

    pub notebook: Notebook,

}



impl TabManager {


    pub fn new(
        notebook: &Notebook
    ) -> Self {


        Self {

            notebook:
                notebook.clone(),

        }

    }




    pub fn new_tab(
        &self,
        name: &str
    ) -> TextView {


        let editor =
            Editor::new(
                name
            );



        let label =
            Label::new(
                Some(name)
            );



        self.notebook
            .append_page(
                &editor.container,
                Some(&label)
            );



        self.notebook
            .set_tab_reorderable(
                &editor.container,
                true
            );



        self.notebook
            .show();



        editor.text_view

    }




    pub fn rename_current(
        &self,
        name: &str
    ) {


        if let Some(page) =
            self.notebook.current_page()
        {


            if let Some(widget) =
                self.notebook.nth_page(
                    Some(page)
                )
            {


                self.notebook
                    .set_tab_label_text(
                        &widget,
                        name
                    );

            }

        }

    }


}
