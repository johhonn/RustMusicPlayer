extern crate gio;
extern crate gtk;
use crate::playlist;
use playlist::Playlist;
use gtk::Image;
use crate::gtk::ImageExt;

use std::env;
use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
        Application,
        ApplicationWindow,
        WidgetExt,
        GtkWindowExt,
        ContainerExt,
        SeparatorToolItem,
        Toolbar,
        ToolButton,
        
       

        };

const PLAY_STOCK: &str = "gtk-media-play";

pub struct MusicToolbar {
   pub open_button: ToolButton,
   pub next_button: ToolButton,
   pub play_button: ToolButton,
   pub  previous_button: ToolButton,
   pub  quit_button: ToolButton,
   pub remove_button: ToolButton,
   pub stop_button: ToolButton,
   pub toolbar: Toolbar,
}

impl MusicToolbar {
    pub fn new() -> Self {
        let toolbar = Toolbar::new();
        let open_button = ToolButton::new_from_stock("gtk-open");
        toolbar.add(&open_button);
        toolbar.add(&SeparatorToolItem::new());
        let previous_button = ToolButton::new_from_stock("gtk-media-previous");
        toolbar.add(&previous_button);
        let play_button = ToolButton::new_from_stock(PLAY_STOCK);
        toolbar.add(&play_button);let stop_button = ToolButton::new_from_stock("gtk-media-stop");
        toolbar.add(&stop_button);
        let next_button = ToolButton::new_from_stock("gtk-media-next");
        toolbar.add(&next_button);
        toolbar.add(&SeparatorToolItem::new());
        let remove_button = ToolButton::new_from_stock("gtk-remove");
        toolbar.add(&remove_button);
        toolbar.add(&SeparatorToolItem::new());
        let quit_button = ToolButton::new_from_stock("gtk-quit");
        toolbar.add(&quit_button);
            MusicToolbar {
                open_button,
                next_button,
                play_button,
                previous_button,
                quit_button,
                remove_button,
                stop_button,
                toolbar
            }
        }
    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
        }
    pub fn set_cover(cover: &Image, playlist: &Playlist) {
            cover.set_from_pixbuf(playlist.pixbuf().as_ref());
            cover.show();
         }   
    }
    
    