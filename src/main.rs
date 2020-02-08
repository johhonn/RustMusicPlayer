
mod playlist;
mod toolbar;

use playlist::Playlist;
use toolbar::MusicToolbar;

extern crate gio;
extern crate gtk;
extern crate gdk_pixbuf;
extern crate id3;
extern crate gtk_sys;
use std::env;
use std::rc::Rc;
use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use crate::gio::FileInfoExt;

use crate::gtk::FileFilterExt;

use crate::gtk::RecentFilterExt;

use crate::gtk::StatusIconExt;
use crate::gtk::DialogExt;
use crate::gtk::FileChooserExt;
use gtk::{
        Application,
        ApplicationWindow,
        WidgetExt,
        GtkWindowExt,
        ContainerExt,
        SeparatorToolItem,
        Toolbar,
        ToolButton,
        Adjustment,
        Image,
        ImageExt,
        Scale,
        ScaleExt,
        ToolButtonExt,
        FileChooserAction, 
        FileChooserDialog, 
        FileFilter
        };
use std::path::PathBuf;
        
use gtk::Orientation::{Horizontal, Vertical};
use gtk_sys::{GTK_RESPONSE_ACCEPT, GTK_RESPONSE_CANCEL};
const RESPONSE_ACCEPT: i32 = GTK_RESPONSE_ACCEPT as i32;
const RESPONSE_CANCEL: i32 = GTK_RESPONSE_CANCEL as i32;
const PLAY_STOCK: &str = "gtk-media-play";
const PAUSE_STOCK: &str = "gtk-media-pause";

struct App {
    adjustment: Adjustment,
    cover: Image,
    playlist: Rc<Playlist>,
    toolbar: MusicToolbar,
    window: ApplicationWindow,
    }
fn show_open_dialog(parent: &ApplicationWindow) -> Option<PathBuf> {
    let mut file = None;
    let dialog = FileChooserDialog::new(Some("Select an MP3 audio
    file"),
    Some(parent), FileChooserAction::Open);
    let filter = FileFilter::new();
    filter.add_mime_type("audio/mp3");
    filter.set_name("MP3 audio file");
    dialog.add_filter(&filter);
    dialog.add_button("Cancel", RESPONSE_CANCEL);
    dialog.add_button("Accept", RESPONSE_ACCEPT);
    let result = dialog.run();
    if result == RESPONSE_ACCEPT {
        file = dialog.get_filename();
    }
    dialog.destroy();
    file
}
impl App {
    fn new(application: Application) -> Self {
        let window = ApplicationWindow::new(&application);
        window.set_title("Rusic");
        let vbox = gtk::Box::new(Vertical, 0);
        
        let toolbar = MusicToolbar::new();
        let playlist = Rc::new(Playlist::new());
        
        let cover = Image::new();
        //cover.set_from_file("cover.jpg");
        
        vbox.add(toolbar.toolbar());
        vbox.add(playlist.view());
       
        
        vbox.add(&cover);
        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        vbox.add(&scale);
        window.add(&vbox);
        
        window.show_all();
        let app = App {
            adjustment,
            playlist,
            cover,
            toolbar,
            window,
            };
       
        app.connect_events();
        app.connect_toolbar_events();
        app
    }
    pub fn connect_toolbar_events(&self) {
       
        let window = self.window.clone();
        self.toolbar.quit_button.connect_clicked(move |_| {
            window.destroy();
        });
        let play_button = self.toolbar.play_button.clone();
       
        let parent = self.window.clone();
        let playlist = self.playlist.clone();
        self.toolbar.open_button.connect_clicked(move |_| {
        let file = show_open_dialog(&parent);
            if let Some(file) = file {
                playlist.add(&file);
            }
        });
        let playlist = self.playlist.clone();
        self.toolbar.remove_button.connect_clicked( move |_| {
        playlist.remove_selection();
        });
        let playlist = self.playlist.clone();
        let cover = self.cover.clone();
        self.toolbar.play_button.connect_clicked(move |_| {
            if play_button.get_stock_id() == Some(PLAY_STOCK.to_string()) {
                play_button.set_stock_id(PAUSE_STOCK);
                toolbar::MusicToolbar::set_cover(&cover, &playlist);
                } 
                else
                {
                play_button.set_stock_id(PLAY_STOCK);
                }
        });

       
    }
    
    fn connect_events(&self) {
    }
   
   
}

fn main() {
    let application = Application::new("test.rust", ApplicationFlags::empty())
        .expect("Application initialization failed");
    application.connect_startup(|application| {
        let window = App::new(application.clone());
        
    });
    application.connect_activate(|_| {});
   
  //  let application=
    application.run(&env::args().collect::<Vec<_>>());
}
