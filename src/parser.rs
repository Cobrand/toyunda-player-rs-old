

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

    /**
     * Sub      : first frame when the sentence appear         -> Sentence
     * Sentence : first frame when the Syllable is highlighted -> Syllable
     *            Sub.sentences[current_frame].value[0].frame == Sub.sentences[current_frame].frame
     *            aka :
     *                  let current_sentence = sub.sentences[sub.current_frame];
     *                  let first_syllable  = sentence.value[0]
     *                  first_syllable.frame == current_sentence.frame
     */

    // TODO : mettre un wrapper qui contient l'image et la couleur avt/aprs

    impl Sentence{
        /// Returns the position of the current syllable
        pub fn current_pos(&mut self) -> Position{
            self.syllables[self.last_colored].value.pos
        }
        /// advance the "pointer" to the last syllable that changed color
        pub fn advance(&mut self, frame_nb: usize) -> Sentence {
            // TODO
            self.last_colored += 1;
            self.clone()
        }
    }

    impl Sub {
        /**
         * Advance the Sub to the frame frame_dest, can't go back
         */
        pub fn advance_to(&mut self, frame_dest: usize) {
            // TODO : bound check
            // TODO : check that we are not going back
            let mut current = &self.sentences[self.current_frame];
            let mut next =  &self.sentences[self.current_frame + 1];

            // we go forward while next is before frame_dest
            while next.frame < frame_dest {
                self.current_frame += 1;
                current = &self.sentences[self.current_frame];
                next =  &self.sentences[self.current_frame + 1];
            };
            // frame_dest is between current.frame and next.frame

            if next.frame == frame_dest { // We are right on the next sentence
                self.current_frame += 1;
            } else if frame_dest > current.frame { // we aren on the current sentence and
                // we have to we have to highlight some syllable
                current.value.advance_to(frame_dest);
            };
        }
    }
    // FIXME : utility ?
    //pub fn sub_builder() {
        //let tmp = Sentence::default();
        //move |frame_nb : FrameNb| {}
    //}
}
