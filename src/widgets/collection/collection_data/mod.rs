/*
YGO Destiny – A Yu-Gi-Oh! sealed draft simulator written in rust.
Copyright (C) 2022  myujiku

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct CollectionData(ObjectSubclass<imp::CollectionData>);
}

impl CollectionData {
    pub fn new(file: &str, name: &str, desc: &str, date: &str, star: bool) -> Self {
        glib::Object::new(&[
            ("file", &file.to_string()),
            ("name", &name.to_string()),
            ("desc", &desc.to_string()),
            ("date", &date.to_string()),
            ("star", &star),
        ])
    }
}
