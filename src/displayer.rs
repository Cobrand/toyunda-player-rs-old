extern crate sdl2_ttf;
use std::vec::Vec;
use std::cmp::Ordering;
use std::path::Path;

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
    fn new(font_path : &Path, ttf_context : &sdl2_ttf::Sdl2TtfContext  ){
        ttf_context.load_font(font_path,72);
    }

    fn get_font_set(&self, index: usize) -> Option<&FontSet> {
        self.fonts.get(index)
    }

    fn add_font_set(&mut self,font_set : FontSet) -> Result<(),&FontSet>{
        let result = self.fonts.binary_search_by(|fontset| fontset.font_size.cmp(&font_set.font_size));
        match result {
            Ok(index) => Err(self.fonts.get(index).unwrap()),
            Err(index) => {self.fonts.insert(index, font_set);Ok(())}
        }
    }

    fn get_closest_font_set(&self,font_size:u32) -> Result<&FontSet,()>{
        match self.fonts.len() {
            0 => Err(()),
            1 => Ok(self.fonts.first().unwrap()),
            _ => {
                let search_result = self.fonts.binary_search_by(|fontset| fontset.font_size.cmp(&font_size));
                match search_result {
                    Ok(index) => Ok(&self.fonts[index]),
                    Err(0) => Ok(&self.fonts[0]),
                    Err(index) =>   if (index == self.fonts.len()) {
                                        Ok(&self.fonts.last().unwrap())
                                    } else {
                                        let font_set_min = &self.fonts[index - 1] ;
                                        let font_set_max = &self.fonts[index] ;
                                        if ( font_set_max.font_size - font_size > font_size - font_set_min.font_size ){
                                            Ok(font_set_min)
                                        } else {
                                            Ok(font_set_max)
                                        }
                                    }
                }
            }
        }
    }
}

pub struct Displayer {
    fonts            : FontList,
}
