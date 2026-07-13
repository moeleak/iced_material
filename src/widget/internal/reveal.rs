use iced_widget::core::time::Instant;

use super::support::AnimatedScalar;
use crate::tokens;

const CLOSED_TARGET: f32 = 0.0;
const OPEN_TARGET: f32 = 1.0;
const VISIBLE_EPSILON: f32 = 0.001;

/// Shared Android-style spatial reveal and effects fade.
#[derive(Debug)]
pub(super) struct RevealAnimation {
    spatial: AnimatedScalar,
    effects: AnimatedScalar,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct RevealFrame {
    pub(super) reveal: f32,
    pub(super) alpha: f32,
    pub(super) is_closing: bool,
}

impl RevealAnimation {
    pub(super) const fn closed() -> Self {
        Self {
            spatial: AnimatedScalar::new(CLOSED_TARGET),
            effects: AnimatedScalar::new(CLOSED_TARGET),
        }
    }

    pub(super) fn open(&mut self, now: Instant) {
        self.spatial
            .set_spring_target(OPEN_TARGET, now, tokens::motion::EXPRESSIVE_SLOW_SPATIAL);
        self.effects
            .set_spring_target(OPEN_TARGET, now, tokens::motion::EXPRESSIVE_FAST_EFFECTS);
    }

    pub(super) fn close(&mut self, now: Instant) {
        self.spatial
            .set_spring_target(CLOSED_TARGET, now, tokens::motion::EXPRESSIVE_FAST_SPATIAL);
        self.effects
            .set_spring_target(CLOSED_TARGET, now, tokens::motion::EXPRESSIVE_FAST_EFFECTS);
    }

    pub(super) fn advance(&mut self, now: Instant) -> bool {
        self.spatial.advance(now) | self.effects.advance(now)
    }

    pub(super) fn is_animating(&self) -> bool {
        self.spatial.is_animating() || self.effects.is_animating()
    }

    pub(super) fn is_visible(&self) -> bool {
        self.is_animating()
            || self.spatial.value > VISIBLE_EPSILON
            || self.effects.value > VISIBLE_EPSILON
            || self.spatial.to > VISIBLE_EPSILON
            || self.effects.to > VISIBLE_EPSILON
    }

    pub(super) fn frame(&self) -> RevealFrame {
        RevealFrame {
            reveal: self.spatial.value.clamp(CLOSED_TARGET, OPEN_TARGET),
            alpha: self.effects.value.clamp(CLOSED_TARGET, OPEN_TARGET),
            is_closing: self.spatial.to <= VISIBLE_EPSILON
                && self.effects.to <= VISIBLE_EPSILON
                && self.is_visible(),
        }
    }
}

impl Default for RevealAnimation {
    fn default() -> Self {
        Self::closed()
    }
}

#[cfg(test)]
#[path = "../../../tests/widget/internal/reveal.rs"]
mod tests;
