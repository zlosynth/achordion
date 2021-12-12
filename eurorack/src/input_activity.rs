use core::mem::discriminant;

use achordion_lib::display::Action as DisplayAction;

pub struct InputActivity {
    last_action: Option<Action>,
}

#[derive(Clone, Copy)]
struct Action {
    pub source: Source,
    pub display_action: DisplayAction,
    pub age: u32,
}

#[derive(Clone, Copy, PartialEq)]
enum Source {
    Pot,
    CV,
    Calibration,
}
use Source::*;

impl InputActivity {
    const SHORT_IDLE: u32 = 1200;
    const LONG_IDLE: u32 = 3200;

    pub fn new() -> Self {
        InputActivity { last_action: None }
    }

    pub fn reconcile<const N1: usize, const N2: usize>(
        &mut self,
        calibration_action: Option<DisplayAction>,
        pot_actions: [Option<DisplayAction>; N1],
        cv_actions: [Option<DisplayAction>; N2],
        fallback_action: DisplayAction,
    ) -> Option<DisplayAction> {
        if let Some(last_action) = self.last_action.as_mut() {
            last_action.age = last_action.age.saturating_add(1);
        }

        // Calibration always takes precedence
        if let Some(display_action) = calibration_action {
            self.last_action = Some(Action {
                source: Calibration,
                display_action,
                age: 0,
            });
            return Some(display_action);
        }

        // Turning a knob overtakes the display, disregarding previous turned
        // pots
        if let Some(display_action) = pot_actions.iter().flatten().next() {
            self.last_action = Some(Action {
                source: Pot,
                display_action: *display_action,
                age: 0,
            });
            return Some(*display_action);
        }

        // If previous action was CV and it is observed again, refresh it
        if let Some(last_action) = self.last_action.as_mut() {
            for display_action in cv_actions.iter().flatten() {
                if discriminant(display_action) == discriminant(&last_action.display_action) {
                    last_action.display_action = *display_action;
                    last_action.age = 0;
                    return Some(*display_action);
                }
            }
        }

        // CV overtakes only aged pots or another CV
        let overtake = if let Some(last_action) = self.last_action {
            last_action.source == Calibration && last_action.age > Self::LONG_IDLE
                || last_action.source == Pot && last_action.age > Self::LONG_IDLE
                || last_action.source == CV && last_action.age > Self::SHORT_IDLE
        } else {
            true
        };
        if overtake {
            if let Some(display_action) = cv_actions.iter().flatten().next() {
                self.last_action = Some(Action {
                    source: CV,
                    display_action: *display_action,
                    age: 0,
                });
                return Some(*display_action);
            }

            return Some(fallback_action);
        }

        Some(self.last_action.unwrap().display_action)
    }
}
