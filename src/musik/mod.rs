use gtk::prelude::*;
use gtk::{self, WidgetExt, TreeViewExt};

struct Column<'a> {
   tree: &'a gtk::TreeView,
   counter: i32
}

impl<'a> Column<'a> {
    fn who(col: &gtk::TreeView) -> Column {
        Column {
            tree: &col,
            counter: 0
        }
    }
    fn add_col(&mut self, title: &str) -> &mut Self {
        let col = gtk::TreeViewColumn::new();
        let render = gtk::CellRendererText::new();
        col.set_title(title);
        col.pack_start(&render, true);
        col.add_attribute(&render, "text", self.counter);
        self.counter += 1;
        self.tree.append_column(&col);
        self
    }
    fn model(&self, model: &[gtk::Type]) -> &Self {
        let model_list = gtk::ListStore::new(model);
        model_list.insert_with_values(None, &[0, 1, 2], &[&"Contoh Lagu", &"Rust loner", &2_3]);
        model_list.insert_with_values(None, &[0, 1, 2], &[&"Saya belajar rust", &"Tanpa nama", &3_3]);
        model_list.insert_with_values(None, &[0, 1, 2], &[&"Cinta gak buta", &"Katanya", &5_1]);
        self.tree.set_model(Some(&model_list));
        self
    }
}

pub fn init(app: &gtk::Application) {
    let main_ui = include_str!("ui/musik.ui");

    let builder = gtk::Builder::new_from_string(main_ui);
    let window: gtk::Window = builder.get_object("musik_ui").unwrap();
    let song_view: gtk::TreeView = builder.get_object("song_tree_view").unwrap();

    let model = [gtk::Type::String
                         ,gtk::Type::String
                         ,gtk::Type::F64];

    Column::who(&song_view)
        .add_col("Song Title")
        .add_col("Artist")
        .add_col("Duration")
        .model(&model);

    song_view.set_headers_visible(true);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}