use crate::modules::SearchResult;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, ListBoxRow, Orientation};

pub struct ResultRow;

impl ResultRow {
    pub fn new(result: &SearchResult) -> ListBoxRow {
        let row = ListBoxRow::new();

        let hbox = GtkBox::new(Orientation::Horizontal, 0);
        hbox.add_css_class("result-row");
        hbox.set_margin_start(16);
        hbox.set_margin_end(16);
        hbox.set_margin_top(8);
        hbox.set_margin_bottom(8);

        // Left side: "AppName — Title"
        let left_text = if let Some(desc) = &result.description {
            let wm_class = desc.split(" • ").next().unwrap_or("");
            format!("{} — {}", wm_class, result.name)
        } else {
            result.name.clone()
        };

        let left_label = Label::new(Some(&left_text));
        left_label.set_halign(gtk4::Align::Start);
        left_label.set_hexpand(true);
        left_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        left_label.add_css_class("result-left");
        hbox.append(&left_label);

        // Right side: "wm_class [workspace]"
        let right_text = if let Some(desc) = &result.description {
            let parts: Vec<&str> = desc.split(" • ").collect();
            if parts.len() == 2 {
                let wm_class = parts[0].to_lowercase();
                let workspace = parts[1].replace("Workspace ", "");
                format!("{} [{}]", wm_class, workspace)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let right_label = Label::new(Some(&right_text));
        right_label.set_halign(gtk4::Align::End);
        right_label.add_css_class("result-right");
        hbox.append(&right_label);

        row.set_child(Some(&hbox));
        row
    }
}
