

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
         * Return the next "state" of the frame and the position of the current "pointed"
         * character at the frame frame_nb
         *
         * Can't go backward (do nothing if frame_nb is back in time)
         */
        pub fn advance(&mut self, frame_nb: usize) -> (Position, /*&*/Pair<Sentence>) {
            // TODO, FIXME : check that sub is advance of the right number of frame
            // TODO : bound check ???
            let mut current = &self.sentences[self.current_frame];
            let next = &self.sentences[self.current_frame+1];

            // Check if we are still in the same sentence
            if frame_nb >= current.frame {
                // we are on the next sentence

                // Check if we are between current and next
                if frame_nb >= next.frame {
                    // we are after the next sentence
                    self.current_frame +=1;
                    current = &self.sentences[self.current_frame];
                };

                // FIXME : tmp = current avant l'update de self.current_frame
                //                      -> comportement voulu ?
                let mut tmp = current.value.clone().advance(frame_nb);
                // FIXME : devrait renvoyer une ref sur un Sub.sentences[idx] ???
                (tmp.current_pos(), Pair{frame: current.frame, value: tmp})
            } else {
                // we are before the current sentence
                self.current_frame += 1;
                let current_sentence = self.sentences[self.current_frame].clone();
                ( current_sentence.value.syllables[current_sentence.value.last_colored].value.pos , current_sentence )
            }
        }

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
