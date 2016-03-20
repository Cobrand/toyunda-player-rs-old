extern crate sdl2_ttf;
use std::vec::Vec;
use std::cmp::Ordering;

struct FontSet {
    font_size:u32,
    font_regular:sdl2_ttf::Font,
    font_bold:sdl2_ttf::Font,
}

impl Eq for FontSet {}

impl PartialEq for FontSet {
    fn eq(&self, other : &Self) -> bool {
        self.font_size.eq(&other.font_size)
    }
}

impl PartialOrd for FontSet {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        self.font_size.partial_cmp(&other.font_size)
    }
}

impl Ord for FontSet {
    fn cmp(&self, other : &Self) -> Ordering {
        self.font_size.cmp(&other.font_size)
    }
}


struct FontList {
    // font list is a SORTED font list
    fonts:Vec<FontSet>
}

impl FontList {
    fn get_font_set(&self, index: usize) -> Option<FontSet> {
        None
    }

    fn add_font_set(&mut self,font_set : FontSet) -> Result<(),&FontSet>{
        let result = self.fonts.binary_search_by(|fontset| fontset.font_size.cmp(&font_set.font_size));
        match result {
            Ok(index) => Err(self.fonts.get(index).unwrap()),
            Err(index) => {self.fonts.insert(index, font_set);Ok(())}
        }
    }

    fn get_closest_font_set(&self){

    }
}

pub struct Displayer {
    fonts            : FontList,
}
