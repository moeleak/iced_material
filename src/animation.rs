//! Small animation helpers for Material themed state.

use iced_widget::core::time::{Duration, Instant};

use crate::{ColorScheme, tokens};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorSchemeTransition {
    from: ColorScheme,
    to: ColorScheme,
    started_at: Instant,
    duration: Duration,
    easing: tokens::motion::CubicBezier,
}

impl ColorSchemeTransition {
    pub fn new(
        from: ColorScheme,
        to: ColorScheme,
        started_at: Instant,
        duration: Duration,
        easing: tokens::motion::CubicBezier,
    ) -> Self {
        Self {
            from,
            to,
            started_at,
            duration,
            easing,
        }
    }

    pub fn material_theme(from: ColorScheme, to: ColorScheme, started_at: Instant) -> Self {
        Self::new(
            from,
            to,
            started_at,
            Duration::from_millis(u64::from(tokens::motion::DURATION_MEDIUM4_MS)),
            tokens::motion::EASING_EMPHASIZED_DECELERATE,
        )
    }

    pub fn value_at(self, now: Instant) -> ColorScheme {
        ColorScheme::interpolate(self.from, self.to, self.eased_progress_at(now))
    }

    pub fn is_finished_at(self, now: Instant) -> bool {
        self.progress_at(now) >= 1.0
    }

    pub fn progress_at(self, now: Instant) -> f32 {
        if self.duration.is_zero() {
            return 1.0;
        }

        (now.saturating_duration_since(self.started_at).as_secs_f32() / self.duration.as_secs_f32())
            .clamp(0.0, 1.0)
    }

    pub fn eased_progress_at(self, now: Instant) -> f32 {
        self.easing.transform(self.progress_at(now))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Theme;

    #[test]
    fn material_theme_transition_uses_m3_easing_and_duration() {
        let start = Instant::now();
        let transition = ColorSchemeTransition::material_theme(
            Theme::Dark.colors(),
            Theme::Light.colors(),
            start,
        );

        assert_eq!(transition.duration, Duration::from_millis(400));
        assert_eq!(
            transition.easing,
            tokens::motion::EASING_EMPHASIZED_DECELERATE
        );
        assert_eq!(transition.progress_at(start), 0.0);
        assert!(!transition.is_finished_at(start + Duration::from_millis(200)));
        assert!(transition.is_finished_at(start + Duration::from_millis(400)));
    }

    #[test]
    fn color_scheme_transition_reaches_target() {
        let start = Instant::now();
        let target = Theme::Light.colors();
        let transition = ColorSchemeTransition::material_theme(Theme::Dark.colors(), target, start);

        assert_eq!(
            transition.value_at(start + Duration::from_millis(500)),
            target
        );
    }
}
