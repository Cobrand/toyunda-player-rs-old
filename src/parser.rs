

mod subtitle {
    use std::vec::Vec;

    /**
     * Coordinate of a point
     */
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Position {
        x: i8,
        y: i8,
    }

    /**
     * RGB color
     */
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Color {
        r: i8,
        g: i8,
        b: i8,
    }

    /**
     * Smallest piece of a text
     */
    #[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Syllable {
        pos: Position,
        word: String,
    }

    /// Frame stamp
    pub type FrameNb = usize;

    /**
     * Pair of FrameNb and type T. Used to
     * associate something with the first frame when it appears.
     */
    #[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Pair<T> {
        pub frame: FrameNb,
        pub value: T,
    }

    /**
     * String printed on the screen.
     * All the string appears, or none of it.
     * Contains a "pointer" to the last Syllable that changed color
     */
    #[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Sentence {
        last_colored: usize,
        // key -> first frame of color transition
        pub syllables: Vec<Pair<Syllable>>,
    }

    /**
     * Subtitle for all the video
     */
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
