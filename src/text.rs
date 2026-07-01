#![allow(dead_code)]
use iced_widget::text::{Catalog, Style, StyleFn};

use crate::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(none)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn none(_: &Theme) -> Style {
    Style { color: None }
}

pub fn primary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().primary.text),
    }
}

pub fn primary_container(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().primary.container_text),
    }
}

pub fn secondary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().secondary.text),
    }
}

pub fn secondary_container(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().secondary.container_text),
    }
}

pub fn tertiary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().tertiary.text),
    }
}

pub fn tertiary_container(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().tertiary.container_text),
    }
}

pub fn error(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().error.text),
    }
}

pub fn error_container(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().error.container_text),
    }
}

pub fn surface(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().surface.text),
    }
}

pub fn surface_variant(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().surface.text_variant),
    }
}

pub fn inverse_surface(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().inverse.inverse_surface_text),
    }
}
