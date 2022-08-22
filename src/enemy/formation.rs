use crate::{WinSize, BASE_SPEED, FORMATION_MEMBERS_MAX};
use bevy::{prelude::Component, time::Time};
use rand::{thread_rng, Rng};

#[derive(Component, Clone)]
pub struct Formation {
    pub start: (f32, f32),
    pub radius: (f32, f32),
    pub pivot: (f32, f32),
    pub speed: f32,
    pub angle: f32,
}

#[derive(Default)]
pub struct FormationMaker {
    current_template: Option<Formation>,
    current_members: u32,
}

impl FormationMaker {
   
    pub fn make(&mut self, win_size: &WinSize) -> Formation {
        match (
            &self.current_template,
            self.current_members >= FORMATION_MEMBERS_MAX,
        ) {
            // if has current template ans still within max member
            (Some(templ), fasle) => {
                self.current_members += 1;
                templ.clone()
            }
            // if first formation or previous formation is null (need to create a new one)
            (None, _) | (_, true) => {
                let mut rng = thread_rng();

                // computer the start x/y
                let w_span = win_size.w / 2. + 100.;
                let h_span = win_size.h / 2. + 100.;
                let x = w_span;
                let y = rng.gen_range(-h_span..h_span) as f32;
                let start = (x, y);

                // computer the pivot x/y
                let w_span = win_size.w / 4.;
                let h_span = win_size.h / 3. - 50.;
                let pivot = (rng.gen_range(-w_span..w_span), rng.gen_range(0.0..h_span));

                // computer the radius
                let radius = (rng.gen_range(80.0..150.), 100.);

                // computer the start angle
                let angle = (y - pivot.1).atan2(x - pivot.0);

                // speed (fixed for now)
                let speed = BASE_SPEED;

                // create Formation
                let formation = Formation {
                    start,
                    pivot,
                    radius,
                    angle,
                    speed,
                };

                // store as template
                self.current_template = Some(formation.clone());

                // reset member to 1
                self.current_members = 1;

                formation
            }
        }
    }
}
impl Formation {



}
