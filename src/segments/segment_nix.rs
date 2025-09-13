use std::env;
use crate::{Powerline, Segment};

pub fn segment_nix(p: &mut Powerline) {
    // TODO: Generalize this to any environment variable?
    if let Ok(val) = env::var("IN_NIX_SHELL") {
        match val.as_str() {
            "impure" => p.segments.push(Segment::new(
                p.theme.nixshell_bg,
                p.theme.nixshell_fg,
                "*"
            )),
            _ => p.segments.push(Segment::new(
                p.theme.nixshell_bg,
                p.theme.nixshell_fg,
                val
            )),
        }
    }
}
