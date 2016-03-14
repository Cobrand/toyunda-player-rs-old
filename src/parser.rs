

mod subtitle {
    use std::vec::Vec;

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

    type FrameNb = usize;

    #[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
    struct Pair<T> {
        pub frame: FrameNb,
        pub value: T,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Sentence {
        last_colored: usize,
        // key -> first frame of color transition
        pub syllables: Vec<Pair<Syllable>>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Sub {
        current_frame: usize,
        // key -> first frame when the sentence appear
        pub sentences: Vec<Pair<Sentence>>,
    }

    impl Sub {
        pub fn advance(&mut self, frame_nb: usize) -> &Pair<Sentence> {
            let mut current = &self.sentences[self.current_frame];
            if frame_nb >= current.frame {
                //current.value.advance(frame_nb);
                current
            } else {
                self.current_frame += 1;
                &self.sentences[self.current_frame]
            }
        }
    }
}
