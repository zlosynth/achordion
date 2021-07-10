use crate::note::Note;

#[derive(Clone, Copy)]
pub enum Action {
    SetChord([i8; 3]),
    SetScaleRoot(Note),
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct State {
    pub led1: bool,
    pub led2: bool,
    pub led3: bool,
    pub led4: bool,
    pub led5: bool,
    pub led6: bool,
    pub led7: bool,
    pub led_sharp: bool,
}

impl From<[bool; 8]> for State {
    fn from(array: [bool; 8]) -> Self {
        Self {
            led1: array[0],
            led2: array[1],
            led3: array[2],
            led4: array[3],
            led5: array[4],
            led6: array[5],
            led7: array[6],
            led_sharp: array[7],
        }
    }
}

impl From<State> for [bool; 8] {
    fn from(state: State) -> Self {
        [
            state.led1,
            state.led2,
            state.led3,
            state.led4,
            state.led5,
            state.led6,
            state.led7,
            state.led_sharp,
        ]
    }
}

pub fn reduce(action: Action) -> State {
    match action {
        Action::SetChord(chord) => reduce_set_chord(chord),
        Action::SetScaleRoot(root) => reduce_set_scale_root(root),
    }
}

fn reduce_set_chord(chord: [i8; 3]) -> State {
    let mut state_array = [false; 8];

    for degree in chord {
        if degree == 0 {
            continue;
        }

        let total_index = if degree > 0 { degree - 1 } else { degree + 1 };

        let index = total_index.rem_euclid(7);

        state_array[index as usize] = true;
    }

    state_array.into()
}

fn reduce_set_scale_root(root: Note) -> State {
    let mut state_array = [false; 8];

    let relative_root = root.to_midi_id() % 12;
    match relative_root {
        0 => {
            state_array[0] = true;
        }
        1 => {
            state_array[0] = true;
            state_array[7] = true;
        }
        2 => {
            state_array[1] = true;
        }
        3 => {
            state_array[1] = true;
            state_array[7] = true;
        }
        4 => {
            state_array[2] = true;
        }
        5 => {
            state_array[3] = true;
        }
        6 => {
            state_array[3] = true;
            state_array[7] = true;
        }
        7 => {
            state_array[4] = true;
        }
        8 => {
            state_array[4] = true;
            state_array[7] = true;
        }
        9 => {
            state_array[5] = true;
        }
        10 => {
            state_array[5] = true;
            state_array[7] = true;
        }
        11 => {
            state_array[6] = true;
        }
        _ => unreachable!(),
    }

    state_array.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_state_from_array() {
        let state: State = [true, false, true, false, true, false, true, false].into();
        assert_eq!(
            state,
            State {
                led1: true,
                led2: false,
                led3: true,
                led4: false,
                led5: true,
                led6: false,
                led7: true,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn convert_state_to_array() {
        let array: [bool; 8] = State {
            led1: true,
            led2: false,
            led3: true,
            led4: false,
            led5: true,
            led6: false,
            led7: true,
            led_sharp: false,
        }
        .into();
        assert_eq!(array, [true, false, true, false, true, false, true, false]);
    }

    #[test]
    fn reduce_single_tone_chord() {
        let state = reduce(Action::SetChord([1, 0, 0]));
        assert_eq!(
            state,
            State {
                led1: true,
                led2: false,
                led3: false,
                led4: false,
                led5: false,
                led6: false,
                led7: false,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_simple_fifth_chord() {
        let state = reduce(Action::SetChord([1, 3, 5]));
        assert_eq!(
            state,
            State {
                led1: true,
                led2: false,
                led3: true,
                led4: false,
                led5: true,
                led6: false,
                led7: false,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_simple_seventh_chord() {
        let state = reduce(Action::SetChord([1, 3, 7]));
        assert_eq!(
            state,
            State {
                led1: true,
                led2: false,
                led3: true,
                led4: false,
                led5: false,
                led6: false,
                led7: true,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_simple_sus_chord() {
        let state = reduce(Action::SetChord([1, 2, 5]));
        assert_eq!(
            state,
            State {
                led1: true,
                led2: true,
                led3: false,
                led4: false,
                led5: true,
                led6: false,
                led7: false,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_chord_with_negative_root() {
        let state = reduce(Action::SetChord([-2, 2, 5]));
        assert_eq!(
            state,
            State {
                led1: false,
                led2: true,
                led3: false,
                led4: false,
                led5: true,
                led6: false,
                led7: true,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_chord_spanning_multiple_octaves_positive() {
        let state = reduce(Action::SetChord([1, 5, 10]));
        assert_eq!(
            state,
            State {
                led1: true,
                led2: false,
                led3: true,
                led4: false,
                led5: true,
                led6: false,
                led7: false,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_chord_spanning_multiple_octaves_negative() {
        let state = reduce(Action::SetChord([1, -3, -13]));
        assert_eq!(
            state,
            State {
                led1: true,
                led2: false,
                led3: true,
                led4: false,
                led5: false,
                led6: true,
                led7: false,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_chord_spanning_multiple_octaves_with_unisono() {
        let state = reduce(Action::SetChord([1, 8, -15]));
        assert_eq!(
            state,
            State {
                led1: true,
                led2: false,
                led3: false,
                led4: false,
                led5: false,
                led6: false,
                led7: false,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_scale_root_c() {
        let state = reduce(Action::SetScaleRoot(Note::C3));
        assert_eq!(
            state,
            State {
                led1: true,
                led2: false,
                led3: false,
                led4: false,
                led5: false,
                led6: false,
                led7: false,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_scale_root_c_sharp() {
        let state = reduce(Action::SetScaleRoot(Note::CSharp3));
        assert_eq!(
            state,
            State {
                led1: true,
                led2: false,
                led3: false,
                led4: false,
                led5: false,
                led6: false,
                led7: false,
                led_sharp: true,
            }
        )
    }

    #[test]
    fn reduce_scale_root_d() {
        let state = reduce(Action::SetScaleRoot(Note::D3));
        assert_eq!(
            state,
            State {
                led1: false,
                led2: true,
                led3: false,
                led4: false,
                led5: false,
                led6: false,
                led7: false,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_scale_root_d_sharp() {
        let state = reduce(Action::SetScaleRoot(Note::DSharp3));
        assert_eq!(
            state,
            State {
                led1: false,
                led2: true,
                led3: false,
                led4: false,
                led5: false,
                led6: false,
                led7: false,
                led_sharp: true,
            }
        )
    }

    #[test]
    fn reduce_scale_root_e() {
        let state = reduce(Action::SetScaleRoot(Note::E3));
        assert_eq!(
            state,
            State {
                led1: false,
                led2: false,
                led3: true,
                led4: false,
                led5: false,
                led6: false,
                led7: false,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_scale_root_f() {
        let state = reduce(Action::SetScaleRoot(Note::F3));
        assert_eq!(
            state,
            State {
                led1: false,
                led2: false,
                led3: false,
                led4: true,
                led5: false,
                led6: false,
                led7: false,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_scale_root_f_sharp() {
        let state = reduce(Action::SetScaleRoot(Note::FSharp3));
        assert_eq!(
            state,
            State {
                led1: false,
                led2: false,
                led3: false,
                led4: true,
                led5: false,
                led6: false,
                led7: false,
                led_sharp: true,
            }
        )
    }

    #[test]
    fn reduce_scale_root_g() {
        let state = reduce(Action::SetScaleRoot(Note::G3));
        assert_eq!(
            state,
            State {
                led1: false,
                led2: false,
                led3: false,
                led4: false,
                led5: true,
                led6: false,
                led7: false,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_scale_root_g_sharp() {
        let state = reduce(Action::SetScaleRoot(Note::GSharp3));
        assert_eq!(
            state,
            State {
                led1: false,
                led2: false,
                led3: false,
                led4: false,
                led5: true,
                led6: false,
                led7: false,
                led_sharp: true,
            }
        )
    }

    #[test]
    fn reduce_scale_root_a() {
        let state = reduce(Action::SetScaleRoot(Note::A3));
        assert_eq!(
            state,
            State {
                led1: false,
                led2: false,
                led3: false,
                led4: false,
                led5: false,
                led6: true,
                led7: false,
                led_sharp: false,
            }
        )
    }

    #[test]
    fn reduce_scale_root_a_sharp() {
        let state = reduce(Action::SetScaleRoot(Note::ASharp3));
        assert_eq!(
            state,
            State {
                led1: false,
                led2: false,
                led3: false,
                led4: false,
                led5: false,
                led6: true,
                led7: false,
                led_sharp: true,
            }
        )
    }

    #[test]
    fn reduce_scale_root_b() {
        let state = reduce(Action::SetScaleRoot(Note::B3));
        assert_eq!(
            state,
            State {
                led1: false,
                led2: false,
                led3: false,
                led4: false,
                led5: false,
                led6: false,
                led7: true,
                led_sharp: false,
            }
        )
    }
}
