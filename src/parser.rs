
mod Subtitle {

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

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Syllable {
        color: Color,
        pos: Position,
        word: String,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Sentence {
        syllables: Vec<Syllable>,
    }

    type frame_nb = i64;

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Sub {
        sentences: HashMap<frame_nb, Sentence>,
    }
}
