

mod subtitle {
    use std::collections::BTreeMap;
    use std::collections::btree_map;

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Position {
        x: i8,
        y: i8,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Color {
        r: i8,
        g: i8,
        b: i8,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Syllable {
        color: Color,
        pos: Position,
        word: String,
    }

    type FrameNb = u64;

    #[derive(Clone)]
    pub struct Sentence<'a> {
        last_colored: btree_map::Iter<'a, FrameNb, Syllable>,
        // key -> first frame of color transition
        syllables: BTreeMap<FrameNb, Syllable>,
    }

    #[derive(Clone)]
    pub struct Sub<'a> {
        current_frame: btree_map::Iter<'a, FrameNb, Sentence<'a>>,
        // key -> first frame when the sentence appear
        sentences: BTreeMap<FrameNb, Sentence<'a>>,
    }

    // impl Sub {
    // pub fn new<'a>() -> Sub<'a>{
    // let mut tmp = Sub {};
    // tmp.current_frame = tmp.sentences.iter();
    // }
    // }
}
