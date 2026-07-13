//! Material 3 style catalogs for the bundled [`crate::Theme`].
//!
//! The modules in this namespace customize iced widget catalogs: colors,
//! borders, shadows, and other visual state. Layout-oriented constructors live
//! under [`crate::widget`].

pub mod badge;
pub mod button;
pub mod checkbox;
pub mod combobox;
pub mod container;
#[cfg(feature = "dialog")]
pub mod dialog;
mod float;
pub mod list;
#[cfg(feature = "markdown")]
pub mod markdown;
pub mod menu;
pub mod pane_grid;
pub mod progress_bar;
#[cfg(feature = "qr_code")]
pub mod qr_code;
pub mod radio;
pub mod rule;
pub mod scrollable;
pub mod select;
#[cfg(feature = "selection")]
pub mod selection;
pub mod slider;
#[cfg(feature = "svg")]
pub mod svg;
pub mod table;
pub mod text_editor;
pub mod text_input;
pub mod toggler;
pub mod tooltip;
