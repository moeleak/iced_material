//! Canvas-based Material 3 progress and loading indicators.

use iced_widget::canvas::{self, Canvas, LineCap, LineJoin, Path, Stroke};
use iced_widget::core::time::{Duration, Instant};
use iced_widget::core::{Color, Length, Point, Rectangle, mouse};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, TAU};

use crate::{Theme, tokens};

const SHAPE_SAMPLE_COUNT: usize = 160;
const CIRCLE_SAMPLE_COUNT: usize = 128;
const ROUNDED_CORNER_SAMPLE_COUNT: usize = 10;
const OUTLINE_LINE_SAMPLE_LENGTH: f32 = 0.06;
const MORPH_OFFSET_CANDIDATE_COUNT: usize = 80;
const MORPH_OFFSET_SAMPLE_COUNT: usize = 32;
const ROUNDED_CORNER_CONTROL_PULL: f32 = 0.38;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LinearMode {
    Determinate,
    Indeterminate,
}

/// A clock-backed state for indeterminate canvas indicators.
#[derive(Debug, Clone)]
pub struct IndeterminateState {
    started_at: Instant,
    elapsed: Duration,
}

impl IndeterminateState {
    /// Creates a new indeterminate animation state.
    pub fn new(started_at: Instant) -> Self {
        Self {
            started_at,
            elapsed: Duration::ZERO,
        }
    }

    /// Advances the state to `now`.
    pub fn advance(&mut self, now: Instant) {
        self.elapsed = now.saturating_duration_since(self.started_at);
    }

    /// Returns the current phase for Material linear progress keyframes.
    pub fn linear_phase(&self) -> f32 {
        elapsed_phase(
            self.elapsed,
            tokens::component::linear_progress::INDETERMINATE_DURATION_MS,
        )
    }

    /// Returns the slower phase used by the four-color linear progress cycle.
    pub fn color_phase(&self) -> f32 {
        elapsed_phase(
            self.elapsed,
            tokens::component::linear_progress::INDETERMINATE_DURATION_MS * 2,
        )
    }

    /// Returns the current phase for expressive loading indicator rotation.
    pub fn loading_phase(&self) -> f32 {
        elapsed_phase(
            self.elapsed,
            tokens::component::loading_indicator::GLOBAL_ROTATION_DURATION_MS,
        )
    }

    /// Indeterminate indicators animate for as long as they are displayed.
    pub const fn is_animating(&self) -> bool {
        true
    }
}

impl Default for IndeterminateState {
    fn default() -> Self {
        Self::new(Instant::now())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LinearProgress {
    mode: LinearMode,
    progress: f32,
    phase: f32,
    color_phase: f32,
    four_color: bool,
}

/// Creates an expressive determinate linear progress indicator.
pub fn linear<'a, Message, Renderer>(
    progress: f32,
    phase: f32,
) -> Canvas<LinearProgress, Message, Theme, Renderer>
where
    Renderer: iced_widget::graphics::geometry::Renderer + 'a,
{
    linear_wavy(progress, phase)
}

/// Creates an expressive determinate linear progress indicator.
pub fn linear_wavy<'a, Message, Renderer>(
    progress: f32,
    phase: f32,
) -> Canvas<LinearProgress, Message, Theme, Renderer>
where
    Renderer: iced_widget::graphics::geometry::Renderer + 'a,
{
    Canvas::new(LinearProgress {
        mode: LinearMode::Determinate,
        progress: progress.clamp(0.0, 1.0),
        phase,
        color_phase: phase,
        four_color: false,
    })
    .width(Length::Fill)
    .height(Length::Fixed(
        tokens::component::linear_progress::WAVE_HEIGHT,
    ))
}

/// Creates a Material indeterminate linear progress indicator.
pub fn linear_indeterminate<'a, Message, Renderer>(
    phase: f32,
    four_color: bool,
) -> Canvas<LinearProgress, Message, Theme, Renderer>
where
    Renderer: iced_widget::graphics::geometry::Renderer + 'a,
{
    Canvas::new(LinearProgress {
        mode: LinearMode::Indeterminate,
        progress: 0.0,
        phase,
        color_phase: phase * 0.5,
        four_color,
    })
    .width(Length::Fill)
    .height(Length::Fixed(
        tokens::component::linear_progress::WAVE_HEIGHT,
    ))
}

/// Creates a Material indeterminate linear progress indicator with explicit color phase.
pub fn linear_indeterminate_with_color_phase<'a, Message, Renderer>(
    phase: f32,
    color_phase: f32,
) -> Canvas<LinearProgress, Message, Theme, Renderer>
where
    Renderer: iced_widget::graphics::geometry::Renderer + 'a,
{
    Canvas::new(LinearProgress {
        mode: LinearMode::Indeterminate,
        progress: 0.0,
        phase,
        color_phase,
        four_color: true,
    })
    .width(Length::Fill)
    .height(Length::Fixed(
        tokens::component::linear_progress::WAVE_HEIGHT,
    ))
}

impl<Message, Renderer> canvas::Program<Message, Theme, Renderer> for LinearProgress
where
    Renderer: iced_widget::graphics::geometry::Renderer,
{
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let colors = theme.colors();

        let active = if self.four_color {
            four_color_indicator(
                colors.primary.color,
                colors.primary.container,
                colors.tertiary.color,
                colors.tertiary.container,
                self.color_phase,
            )
        } else {
            colors.primary.color
        };

        let track = colors.surface.container.highest;

        match self.mode {
            LinearMode::Determinate => {
                draw_linear_determinate_track(&mut frame, track, active, self.progress);
                draw_linear_determinate(&mut frame, active, self.progress, self.phase);
            }
            LinearMode::Indeterminate => {
                let bars = indeterminate_bars(self.phase);

                draw_linear_indeterminate_track(&mut frame, track, &bars);

                for (index, bar) in bars.into_iter().enumerate() {
                    draw_indeterminate_bar(
                        &mut frame,
                        active,
                        bar,
                        self.phase + index as f32 * 0.25,
                    );
                }
            }
        }

        vec![frame.into_geometry()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoadingMode {
    Uncontained,
    Contained,
}

#[derive(Debug, Clone, Copy)]
pub struct LoadingIndicator {
    mode: LoadingMode,
    progress: Option<f32>,
    phase: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LoadingVertex {
    point: Point,
    rounding: f32,
}

impl LoadingVertex {
    fn new(x: f32, y: f32, rounding: f32) -> Self {
        Self {
            point: Point::new(x, y),
            rounding,
        }
    }
}

/// Creates an expressive indeterminate loading indicator.
pub fn loading_indicator<'a, Message, Renderer>(
    phase: f32,
) -> Canvas<LoadingIndicator, Message, Theme, Renderer>
where
    Renderer: iced_widget::graphics::geometry::Renderer + 'a,
{
    Canvas::new(LoadingIndicator {
        mode: LoadingMode::Uncontained,
        progress: None,
        phase,
    })
    .width(Length::Fixed(
        tokens::component::loading_indicator::CONTAINER_WIDTH,
    ))
    .height(Length::Fixed(
        tokens::component::loading_indicator::CONTAINER_HEIGHT,
    ))
}

/// Creates an expressive contained indeterminate loading indicator.
pub fn contained_loading_indicator<'a, Message, Renderer>(
    phase: f32,
) -> Canvas<LoadingIndicator, Message, Theme, Renderer>
where
    Renderer: iced_widget::graphics::geometry::Renderer + 'a,
{
    Canvas::new(LoadingIndicator {
        mode: LoadingMode::Contained,
        progress: None,
        phase,
    })
    .width(Length::Fixed(
        tokens::component::loading_indicator::CONTAINER_WIDTH,
    ))
    .height(Length::Fixed(
        tokens::component::loading_indicator::CONTAINER_HEIGHT,
    ))
}

/// Creates an expressive determinate loading indicator.
pub fn determinate_loading_indicator<'a, Message, Renderer>(
    progress: f32,
) -> Canvas<LoadingIndicator, Message, Theme, Renderer>
where
    Renderer: iced_widget::graphics::geometry::Renderer + 'a,
{
    Canvas::new(LoadingIndicator {
        mode: LoadingMode::Uncontained,
        progress: Some(progress.clamp(0.0, 1.0)),
        phase: 0.0,
    })
    .width(Length::Fixed(
        tokens::component::loading_indicator::CONTAINER_WIDTH,
    ))
    .height(Length::Fixed(
        tokens::component::loading_indicator::CONTAINER_HEIGHT,
    ))
}

/// Creates an expressive contained determinate loading indicator.
pub fn determinate_contained_loading_indicator<'a, Message, Renderer>(
    progress: f32,
) -> Canvas<LoadingIndicator, Message, Theme, Renderer>
where
    Renderer: iced_widget::graphics::geometry::Renderer + 'a,
{
    Canvas::new(LoadingIndicator {
        mode: LoadingMode::Contained,
        progress: Some(progress.clamp(0.0, 1.0)),
        phase: 0.0,
    })
    .width(Length::Fixed(
        tokens::component::loading_indicator::CONTAINER_WIDTH,
    ))
    .height(Length::Fixed(
        tokens::component::loading_indicator::CONTAINER_HEIGHT,
    ))
}

impl<Message, Renderer> canvas::Program<Message, Theme, Renderer> for LoadingIndicator
where
    Renderer: iced_widget::graphics::geometry::Renderer,
{
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let colors = theme.colors();

        let (container, active) = match self.mode {
            LoadingMode::Uncontained => (None, colors.primary.color),
            LoadingMode::Contained => (
                Some(colors.primary.container),
                colors.primary.container_text,
            ),
        };

        if let Some(color) = container {
            let container = Path::circle(frame.center(), frame.width().min(frame.height()) / 2.0);
            frame.fill(&container, color);
        }

        let radius = frame.width().min(frame.height()) * loading_shape_scale() / 2.0;
        let path = if let Some(progress) = self.progress {
            determinate_loading_shape_path(frame.center(), radius, progress)
        } else {
            loading_shape_path(frame.center(), radius, self.phase)
        };
        frame.fill(&path, active);

        vec![frame.into_geometry()]
    }
}

fn elapsed_phase(elapsed: Duration, duration_ms: u16) -> f32 {
    let duration = f32::from(duration_ms) / 1000.0;

    if duration <= 0.0 {
        return 0.0;
    }

    (elapsed.as_secs_f32() / duration).rem_euclid(1.0)
}

fn draw_linear_determinate_track<Renderer>(
    frame: &mut canvas::Frame<Renderer>,
    track: Color,
    stop: Color,
    progress: f32,
) where
    Renderer: iced_widget::graphics::geometry::Renderer,
{
    let width = frame.width();
    let height = frame.height();
    let y = height / 2.0;
    let stroke_width = tokens::component::linear_progress::TRACK_THICKNESS;
    let left = stroke_width / 2.0;
    let stop_size = tokens::component::linear_progress::STOP_SIZE;
    let stop_center_x =
        width - tokens::component::linear_progress::STOP_TRAILING_SPACE - stop_size / 2.0;
    let right = (stop_center_x - stop_size / 2.0).max(left);
    let active_end = left + (right - left) * progress.clamp(0.0, 1.0);
    let track_start =
        (active_end + tokens::component::linear_progress::TRACK_ACTIVE_SPACE + stroke_width)
            .clamp(left, right);

    if track_start < right {
        frame.stroke(
            &Path::line(Point::new(track_start, y), Point::new(right, y)),
            round_stroke(track, stroke_width),
        );
    }

    let stop_radius = linear_stop_radius(progress, width);
    if stop_radius > 0.0 {
        frame.fill(
            &Path::circle(Point::new(stop_center_x, y), stop_radius),
            stop,
        );
    }
}

fn draw_linear_indeterminate_track<Renderer>(
    frame: &mut canvas::Frame<Renderer>,
    track: Color,
    bars: &[IndeterminateBar; 2],
) where
    Renderer: iced_widget::graphics::geometry::Renderer,
{
    let stroke_width = tokens::component::linear_progress::TRACK_THICKNESS;
    let left = stroke_width / 2.0;
    let right = frame.width() - stroke_width / 2.0;
    let y = frame.height() / 2.0;
    let gap = tokens::component::linear_progress::TRACK_ACTIVE_SPACE + stroke_width;
    let mut cursor = left;

    let mut ranges = [
        linear_bar_range(bars[0], left, right),
        linear_bar_range(bars[1], left, right),
    ];
    ranges.sort_by(|a, b| a.0.total_cmp(&b.0));

    for (start, end) in ranges {
        if end <= start {
            continue;
        }

        let track_end = (start - gap).clamp(left, right);
        if track_end > cursor {
            frame.stroke(
                &Path::line(Point::new(cursor, y), Point::new(track_end, y)),
                round_stroke(track, stroke_width),
            );
        }

        cursor = cursor.max((end + gap).clamp(left, right));
    }

    if cursor < right {
        frame.stroke(
            &Path::line(Point::new(cursor, y), Point::new(right, y)),
            round_stroke(track, stroke_width),
        );
    }
}

fn linear_stop_radius(progress: f32, width: f32) -> f32 {
    let stop_size = tokens::component::linear_progress::STOP_SIZE;
    let stroke_width = tokens::component::linear_progress::TRACK_THICKNESS;
    let stop_x = width - tokens::component::linear_progress::STOP_TRAILING_SPACE - stop_size;
    let progress_x = width * progress.clamp(0.0, 1.0) + stroke_width / 2.0;
    let size = if stop_x <= progress_x {
        (stop_size - (progress_x - stop_x)).max(0.0)
    } else {
        stop_size
    };

    size / 2.0
}

fn draw_linear_determinate<Renderer>(
    frame: &mut canvas::Frame<Renderer>,
    active: Color,
    progress: f32,
    phase: f32,
) where
    Renderer: iced_widget::graphics::geometry::Renderer,
{
    let stroke_width = tokens::component::linear_progress::ACTIVE_INDICATOR_HEIGHT;
    let left = stroke_width / 2.0;
    let right = frame.width()
        - tokens::component::linear_progress::STOP_TRAILING_SPACE
        - tokens::component::linear_progress::STOP_SIZE;
    let end = left + (right - left).max(0.0) * progress.clamp(0.0, 1.0);
    let amplitude = tokens::component::linear_progress::ACTIVE_WAVE_AMPLITUDE
        * determinate_wave_amplitude(progress);

    if end <= left {
        return;
    }

    let path = wave_path(
        left,
        end,
        frame.height() / 2.0,
        amplitude,
        tokens::component::linear_progress::ACTIVE_WAVE_WAVELENGTH,
        phase,
    );
    frame.stroke(&path, round_stroke(active, stroke_width));
}

fn draw_indeterminate_bar<Renderer>(
    frame: &mut canvas::Frame<Renderer>,
    active: Color,
    bar: IndeterminateBar,
    wave_phase: f32,
) where
    Renderer: iced_widget::graphics::geometry::Renderer,
{
    let stroke_width = tokens::component::linear_progress::ACTIVE_INDICATOR_HEIGHT;
    let left = stroke_width / 2.0;
    let right = frame.width() - stroke_width / 2.0;
    let (start, end) = linear_bar_range(bar, left, right);

    if end <= start {
        return;
    }

    let path = wave_path(
        start,
        end,
        frame.height() / 2.0,
        tokens::component::linear_progress::ACTIVE_WAVE_AMPLITUDE,
        tokens::component::linear_progress::INDETERMINATE_ACTIVE_WAVE_WAVELENGTH,
        wave_phase,
    );

    frame.stroke(&path, round_stroke(active, stroke_width));
}

fn determinate_wave_amplitude(progress: f32) -> f32 {
    let progress = progress.clamp(0.0, 1.0);

    if progress <= 0.1 || progress >= 0.95 {
        0.0
    } else {
        1.0
    }
}

fn linear_bar_range(bar: IndeterminateBar, left: f32, right: f32) -> (f32, f32) {
    let width = (right - left).max(0.0);
    let start = left + width * bar.tail.clamp(0.0, 1.0);
    let end = left + width * bar.head.clamp(0.0, 1.0);

    if end >= start {
        (start, end)
    } else {
        (end, start)
    }
}

fn round_stroke(color: Color, width: f32) -> Stroke<'static> {
    Stroke::default()
        .with_color(color)
        .with_width(width)
        .with_line_cap(LineCap::Round)
        .with_line_join(LineJoin::Round)
}

fn wave_path(start: f32, end: f32, y: f32, amplitude: f32, wavelength: f32, phase: f32) -> Path {
    let length = (end - start).max(0.0);
    let step = 3.0_f32.max(wavelength / 12.0);

    Path::new(|path| {
        path.move_to(Point::new(
            start,
            y + wave_offset(0.0, amplitude, wavelength, phase),
        ));

        let mut distance = step;
        while distance < length {
            let x = start + distance;
            path.line_to(Point::new(
                x,
                y + wave_offset(distance, amplitude, wavelength, phase),
            ));
            distance += step;
        }

        path.line_to(Point::new(
            end,
            y + wave_offset(length, amplitude, wavelength, phase),
        ));
    })
}

fn wave_offset(distance: f32, amplitude: f32, wavelength: f32, phase: f32) -> f32 {
    if wavelength <= 0.0 {
        return 0.0;
    }

    ((distance / wavelength) * TAU + phase.rem_euclid(1.0) * TAU).sin() * amplitude
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct IndeterminateBar {
    tail: f32,
    head: f32,
}

fn indeterminate_bars(phase: f32) -> [IndeterminateBar; 2] {
    [
        IndeterminateBar {
            tail: indeterminate_keyframe_progress(
                phase,
                tokens::component::linear_progress::FIRST_LINE_TAIL_DELAY_MS,
                tokens::component::linear_progress::FIRST_LINE_TAIL_DURATION_MS,
            ),
            head: indeterminate_keyframe_progress(
                phase,
                tokens::component::linear_progress::FIRST_LINE_HEAD_DELAY_MS,
                tokens::component::linear_progress::FIRST_LINE_HEAD_DURATION_MS,
            ),
        },
        IndeterminateBar {
            tail: indeterminate_keyframe_progress(
                phase,
                tokens::component::linear_progress::SECOND_LINE_TAIL_DELAY_MS,
                tokens::component::linear_progress::SECOND_LINE_TAIL_DURATION_MS,
            ),
            head: indeterminate_keyframe_progress(
                phase,
                tokens::component::linear_progress::SECOND_LINE_HEAD_DELAY_MS,
                tokens::component::linear_progress::SECOND_LINE_HEAD_DURATION_MS,
            ),
        },
    ]
}

fn indeterminate_keyframe_progress(phase: f32, delay_ms: u16, duration_ms: u16) -> f32 {
    let elapsed_ms = phase.rem_euclid(1.0)
        * f32::from(tokens::component::linear_progress::INDETERMINATE_DURATION_MS);
    let delay_ms = f32::from(delay_ms);
    let duration_ms = f32::from(duration_ms);

    if elapsed_ms <= delay_ms {
        return 0.0;
    }

    if elapsed_ms >= delay_ms + duration_ms {
        return 1.0;
    }

    tokens::motion::EASING_EMPHASIZED_ACCELERATE.transform((elapsed_ms - delay_ms) / duration_ms)
}

fn four_color_indicator(
    primary: Color,
    primary_container: Color,
    tertiary: Color,
    tertiary_container: Color,
    phase: f32,
) -> Color {
    let phase = phase.rem_euclid(1.0);

    if !(0.15..0.25).contains(&phase)
        && !(0.40..0.50).contains(&phase)
        && !(0.65..0.75).contains(&phase)
        && !(0.90..1.0).contains(&phase)
    {
        if phase < 0.25 || phase >= 0.90 {
            return primary;
        }
        if phase < 0.50 {
            return primary_container;
        }
        if phase < 0.75 {
            return tertiary;
        }

        return tertiary_container;
    }

    if phase < 0.25 {
        color_lerp(primary, primary_container, (phase - 0.15) / 0.10)
    } else if phase < 0.50 {
        color_lerp(primary_container, tertiary, (phase - 0.40) / 0.10)
    } else if phase < 0.75 {
        color_lerp(tertiary, tertiary_container, (phase - 0.65) / 0.10)
    } else {
        color_lerp(tertiary_container, primary, (phase - 0.90) / 0.10)
    }
}

fn color_lerp(from: Color, to: Color, progress: f32) -> Color {
    let progress = progress.clamp(0.0, 1.0);

    Color {
        r: from.r + (to.r - from.r) * progress,
        g: from.g + (to.g - from.g) * progress,
        b: from.b + (to.b - from.b) * progress,
        a: from.a + (to.a - from.a) * progress,
    }
}

fn loading_shape_path(center: Point, radius: f32, phase: f32) -> Path {
    let phase = phase.rem_euclid(1.0);
    let morph_position = (phase
        * f32::from(tokens::component::loading_indicator::GLOBAL_ROTATION_DURATION_MS)
        / f32::from(tokens::component::loading_indicator::MORPH_INTERVAL_MS))
    .rem_euclid(tokens::component::loading_indicator::INDETERMINATE_SHAPE_COUNT as f32);
    let from_index = morph_position.floor() as usize;
    let to_index =
        (from_index + 1) % tokens::component::loading_indicator::INDETERMINATE_SHAPE_COUNT;
    let local_progress = morph_position.fract();
    let morph_progress = loading_spring_progress(local_progress);
    let rotation = phase * TAU + (from_index as f32 + 1.0 + morph_progress) * FRAC_PI_2;
    let from = material_loading_outline(from_index);
    let to = material_loading_outline(to_index);

    morphed_loading_shape_path(&from, &to, center, radius, morph_progress, rotation)
}

fn determinate_loading_shape_path(center: Point, radius: f32, progress: f32) -> Path {
    let progress = progress.clamp(0.0, 1.0);
    let from = determinate_loading_outline(0);
    let to = determinate_loading_outline(1);
    let rotation = -progress * std::f32::consts::PI;

    morphed_loading_shape_path(&from, &to, center, radius, progress, rotation)
}

fn morphed_loading_shape_path(
    from: &[Point],
    to: &[Point],
    center: Point,
    radius: f32,
    morph_progress: f32,
    rotation: f32,
) -> Path {
    let points = morphed_loading_points(from, to, morph_progress);

    closed_polyline_path(&points, center, radius, rotation)
}

fn morphed_loading_points(from: &[Point], to: &[Point], morph_progress: f32) -> Vec<Point> {
    let morph_progress = morph_progress.clamp(0.0, 1.0);
    let from = MeasuredOutline::new(from);
    let to = MeasuredOutline::new(to);
    let to_offset = best_outline_offset(&from, &to);
    let mut points = Vec::with_capacity(SHAPE_SAMPLE_COUNT);

    for index in 0..SHAPE_SAMPLE_COUNT {
        let progress = index as f32 / SHAPE_SAMPLE_COUNT as f32;
        let from = from.point_at(progress);
        let to = to.point_at(progress + to_offset);

        points.push(Point::new(
            from.x + (to.x - from.x) * morph_progress,
            from.y + (to.y - from.y) * morph_progress,
        ));
    }

    center_points_on_bounds(&points)
}

#[derive(Debug)]
struct MeasuredOutline<'a> {
    points: &'a [Point],
    cumulative_lengths: Vec<f32>,
    perimeter: f32,
}

impl<'a> MeasuredOutline<'a> {
    fn new(points: &'a [Point]) -> Self {
        if points.len() < 2 {
            return Self {
                points,
                cumulative_lengths: vec![0.0],
                perimeter: 0.0,
            };
        }

        let mut cumulative_lengths = Vec::with_capacity(points.len() + 1);
        let mut perimeter = 0.0;

        cumulative_lengths.push(0.0);

        for index in 0..points.len() {
            perimeter += distance(points[index], points[(index + 1) % points.len()]);
            cumulative_lengths.push(perimeter);
        }

        Self {
            points,
            cumulative_lengths,
            perimeter,
        }
    }

    fn point_at(&self, progress: f32) -> Point {
        if self.points.is_empty() {
            return Point::ORIGIN;
        }

        if self.points.len() == 1 || self.perimeter <= f32::EPSILON {
            return self.points[0];
        }

        let target = progress.rem_euclid(1.0) * self.perimeter;
        let segment_index = self
            .cumulative_lengths
            .partition_point(|length| *length < target)
            .saturating_sub(1)
            .min(self.points.len() - 1);
        let segment_start = self.cumulative_lengths[segment_index];
        let segment_end = self.cumulative_lengths[segment_index + 1];
        let segment_length = (segment_end - segment_start).max(f32::EPSILON);
        let local_progress = (target - segment_start) / segment_length;

        point_lerp(
            self.points[segment_index],
            self.points[(segment_index + 1) % self.points.len()],
            local_progress,
        )
    }
}

fn best_outline_offset(from: &MeasuredOutline<'_>, to: &MeasuredOutline<'_>) -> f32 {
    let mut best_offset = 0.0;
    let mut best_distance = f32::INFINITY;

    for candidate in 0..MORPH_OFFSET_CANDIDATE_COUNT {
        let offset = candidate as f32 / MORPH_OFFSET_CANDIDATE_COUNT as f32;
        let mut candidate_distance = 0.0;

        for sample in 0..MORPH_OFFSET_SAMPLE_COUNT {
            let progress = sample as f32 / MORPH_OFFSET_SAMPLE_COUNT as f32;
            let from = from.point_at(progress);
            let to = to.point_at(progress + offset);

            candidate_distance += squared_distance(from, to);
        }

        if candidate_distance < best_distance {
            best_distance = candidate_distance;
            best_offset = offset;
        }
    }

    best_offset
}

fn loading_spring_progress(progress: f32) -> f32 {
    let seconds = progress.clamp(0.0, 1.0)
        * f32::from(tokens::component::loading_indicator::MORPH_INTERVAL_MS)
        / 1000.0;
    let damping_ratio = tokens::component::loading_indicator::MORPH_SPRING_DAMPING_RATIO;
    let stiffness = tokens::component::loading_indicator::MORPH_SPRING_STIFFNESS;
    let natural_frequency = stiffness.sqrt();

    if damping_ratio >= 1.0 {
        return (1.0 - (-natural_frequency * seconds).exp()).clamp(0.0, 1.0);
    }

    let damped_frequency = natural_frequency * (1.0 - damping_ratio * damping_ratio).sqrt();
    let envelope = (-damping_ratio * natural_frequency * seconds).exp();
    let phase = damped_frequency * seconds;
    let response = 1.0
        - envelope
            * (phase.cos()
                + damping_ratio / (1.0 - damping_ratio * damping_ratio).sqrt() * phase.sin());

    response.clamp(0.0, 1.0)
}

fn closed_polyline_path(points: &[Point], center: Point, radius: f32, rotation: f32) -> Path {
    if points.is_empty() {
        return Path::new(|_| {});
    }

    Path::new(|path| {
        path.move_to(place_loading_point(points[0], center, radius, rotation));

        for point in &points[1..] {
            path.line_to(place_loading_point(*point, center, radius, rotation));
        }

        path.close();
    })
}

fn place_loading_point(point: Point, center: Point, radius: f32, rotation: f32) -> Point {
    let cos = rotation.cos();
    let sin = rotation.sin();
    let x = point.x * cos - point.y * sin;
    let y = point.x * sin + point.y * cos;

    Point::new(center.x + x * radius, center.y + y * radius)
}

fn loading_shape_scale() -> f32 {
    let mut max_radius = 1.0_f32;

    for shape_index in 0..tokens::component::loading_indicator::INDETERMINATE_SHAPE_COUNT {
        for point in center_points_on_bounds(&material_loading_outline(shape_index)) {
            max_radius = max_radius.max(point.x.hypot(point.y));
        }
    }

    tokens::component::loading_indicator::ACTIVE_INDICATOR_SCALE / max_radius
}

fn material_loading_outline(shape_index: usize) -> Vec<Point> {
    rounded_outline(&material_loading_vertices(shape_index))
}

fn determinate_loading_outline(shape_index: usize) -> Vec<Point> {
    match shape_index % tokens::component::loading_indicator::DETERMINATE_SHAPE_COUNT {
        0 => rounded_outline(&circle_vertices(CIRCLE_SAMPLE_COUNT, TAU / 20.0)),
        _ => material_loading_outline(0),
    }
}

fn material_loading_vertices(shape_index: usize) -> Vec<LoadingVertex> {
    normalize_vertices(
        match shape_index % tokens::component::loading_indicator::INDETERMINATE_SHAPE_COUNT {
            0 => custom_loading_polygon(
                &[
                    LoadingVertex::new(0.193, 0.277, 0.053),
                    LoadingVertex::new(0.176, 0.055, 0.053),
                ],
                10,
                false,
            ),
            1 => star_vertices(9, 0.8, -FRAC_PI_2, 0.5),
            2 => custom_loading_polygon(
                &[
                    LoadingVertex::new(0.500, -0.009, 0.172),
                    LoadingVertex::new(1.030, 0.365, 0.164),
                    LoadingVertex::new(0.828, 0.970, 0.169),
                ],
                1,
                true,
            ),
            3 => custom_loading_polygon(
                &[
                    LoadingVertex::new(0.961, 0.039, 0.426),
                    LoadingVertex::new(1.001, 0.428, 0.0),
                    LoadingVertex::new(1.000, 0.609, 1.0),
                ],
                2,
                true,
            ),
            4 => star_vertices(8, 0.8, 0.0, 0.15),
            5 => custom_loading_polygon(
                &[
                    LoadingVertex::new(1.237, 1.236, 0.258),
                    LoadingVertex::new(0.500, 0.918, 0.233),
                ],
                4,
                false,
            ),
            _ => ellipse_vertices(CIRCLE_SAMPLE_COUNT, 1.0, 0.64, -FRAC_PI_4),
        },
    )
}

#[cfg(test)]
fn determinate_loading_vertices(shape_index: usize) -> Vec<LoadingVertex> {
    match shape_index % tokens::component::loading_indicator::DETERMINATE_SHAPE_COUNT {
        0 => circle_vertices(CIRCLE_SAMPLE_COUNT, TAU / 20.0),
        _ => material_loading_vertices(0),
    }
}

fn circle_vertices(count: usize, rotation: f32) -> Vec<LoadingVertex> {
    let mut vertices = Vec::with_capacity(count);

    for index in 0..count {
        let angle = index as f32 * TAU / count as f32 + rotation;

        vertices.push(LoadingVertex::new(angle.cos(), angle.sin(), 0.0));
    }

    vertices
}

fn custom_loading_polygon(
    points: &[LoadingVertex],
    reps: usize,
    mirroring: bool,
) -> Vec<LoadingVertex> {
    let center = Point::new(0.5, 0.5);

    if mirroring {
        let angles: Vec<f32> = points
            .iter()
            .map(|vertex| (vertex.point.y - center.y).atan2(vertex.point.x - center.x))
            .collect();
        let distances: Vec<f32> = points
            .iter()
            .map(|vertex| (vertex.point.x - center.x).hypot(vertex.point.y - center.y))
            .collect();
        let actual_reps = reps * 2;
        let section_angle = TAU / actual_reps as f32;
        let mut vertices = Vec::with_capacity(points.len() * actual_reps);

        for rep in 0..actual_reps {
            for index in 0..points.len() {
                let mirrored = rep % 2 == 1;
                let source = if mirrored {
                    points.len() - 1 - index
                } else {
                    index
                };

                if source > 0 || !mirrored {
                    let angle = section_angle * rep as f32
                        + if mirrored {
                            section_angle - angles[source] + 2.0 * angles[0]
                        } else {
                            angles[source]
                        };

                    vertices.push(LoadingVertex::new(
                        center.x + angle.cos() * distances[source],
                        center.y + angle.sin() * distances[source],
                        points[source].rounding,
                    ));
                }
            }
        }

        vertices
    } else {
        let mut vertices = Vec::with_capacity(points.len() * reps);

        for rep in 0..reps {
            let angle = rep as f32 * TAU / reps as f32;
            let cos = angle.cos();
            let sin = angle.sin();

            for vertex in points {
                let offset_x = vertex.point.x - center.x;
                let offset_y = vertex.point.y - center.y;

                vertices.push(LoadingVertex::new(
                    center.x + offset_x * cos - offset_y * sin,
                    center.y + offset_x * sin + offset_y * cos,
                    vertex.rounding,
                ));
            }
        }

        vertices
    }
}

fn star_vertices(
    points_per_radius: usize,
    inner_radius: f32,
    rotation: f32,
    rounding: f32,
) -> Vec<LoadingVertex> {
    let count = points_per_radius * 2;
    let mut vertices = Vec::with_capacity(count);

    for index in 0..count {
        let radius = if index % 2 == 0 { 1.0 } else { inner_radius };
        let angle = index as f32 * TAU / count as f32 + rotation;

        vertices.push(LoadingVertex::new(
            angle.cos() * radius,
            angle.sin() * radius,
            rounding,
        ));
    }

    vertices
}

fn ellipse_vertices(
    count: usize,
    x_radius: f32,
    y_radius: f32,
    rotation: f32,
) -> Vec<LoadingVertex> {
    let mut vertices = Vec::with_capacity(count);
    let cos = rotation.cos();
    let sin = rotation.sin();

    for index in 0..count {
        let angle = index as f32 * TAU / count as f32;
        let x = angle.cos() * x_radius;
        let y = angle.sin() * y_radius;

        vertices.push(LoadingVertex::new(
            x * cos - y * sin,
            x * sin + y * cos,
            0.0,
        ));
    }

    vertices
}

fn normalize_vertices(vertices: Vec<LoadingVertex>) -> Vec<LoadingVertex> {
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for vertex in &vertices {
        min_x = min_x.min(vertex.point.x);
        max_x = max_x.max(vertex.point.x);
        min_y = min_y.min(vertex.point.y);
        max_y = max_y.max(vertex.point.y);
    }

    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;
    let scale = ((max_x - min_x).max(max_y - min_y) / 2.0).max(f32::EPSILON);

    vertices
        .into_iter()
        .map(|vertex| LoadingVertex {
            point: Point::new(
                (vertex.point.x - center_x) / scale,
                (vertex.point.y - center_y) / scale,
            ),
            rounding: vertex.rounding / scale,
        })
        .collect()
}

fn rounded_outline(vertices: &[LoadingVertex]) -> Vec<Point> {
    if vertices.len() < 3 {
        return vertices.iter().map(|vertex| vertex.point).collect();
    }

    let corners: Vec<RoundedCorner> = (0..vertices.len())
        .map(|index| rounded_corner(vertices, index))
        .collect();
    let mut outline = Vec::with_capacity(vertices.len() * (ROUNDED_CORNER_SAMPLE_COUNT + 2));

    for index in 0..corners.len() {
        let previous_end = corners[(index + corners.len() - 1) % corners.len()].end;
        let corner = corners[index];

        append_line_samples(&mut outline, previous_end, corner.start);

        if corner.cut <= f32::EPSILON {
            push_outline_point(&mut outline, corner.control);
        } else {
            append_quadratic_samples(&mut outline, corner.start, corner.control, corner.end);
        }
    }

    outline
}

#[derive(Debug, Clone, Copy)]
struct RoundedCorner {
    start: Point,
    control: Point,
    end: Point,
    cut: f32,
}

fn rounded_corner(vertices: &[LoadingVertex], index: usize) -> RoundedCorner {
    let count = vertices.len();
    let previous = vertices[(index + count - 1) % count].point;
    let current = vertices[index].point;
    let next = vertices[(index + 1) % count].point;
    let incoming = point_sub(previous, current);
    let outgoing = point_sub(next, current);
    let incoming_length = point_length(incoming);
    let outgoing_length = point_length(outgoing);
    let cut = vertices[index]
        .rounding
        .max(0.0)
        .min(incoming_length * 0.45)
        .min(outgoing_length * 0.45);

    if cut <= f32::EPSILON || incoming_length <= f32::EPSILON || outgoing_length <= f32::EPSILON {
        return RoundedCorner {
            start: current,
            control: current,
            end: current,
            cut: 0.0,
        };
    }

    let start = point_add(current, point_scale(incoming, cut / incoming_length));
    let end = point_add(current, point_scale(outgoing, cut / outgoing_length));
    let chord_midpoint = point_lerp(start, end, 0.5);
    let control = point_lerp(
        chord_midpoint,
        current,
        ROUNDED_CORNER_CONTROL_PULL.clamp(0.0, 1.0),
    );

    RoundedCorner {
        start,
        control,
        end,
        cut,
    }
}

fn append_line_samples(outline: &mut Vec<Point>, from: Point, to: Point) {
    let length = distance(from, to);
    let steps = (length / OUTLINE_LINE_SAMPLE_LENGTH).ceil().max(1.0) as usize;

    if outline.is_empty() {
        push_outline_point(outline, from);
    }

    for step in 1..=steps {
        push_outline_point(outline, point_lerp(from, to, step as f32 / steps as f32));
    }
}

fn append_quadratic_samples(outline: &mut Vec<Point>, from: Point, control: Point, to: Point) {
    for step in 1..=ROUNDED_CORNER_SAMPLE_COUNT {
        let progress = step as f32 / ROUNDED_CORNER_SAMPLE_COUNT as f32;
        let first = point_lerp(from, control, progress);
        let second = point_lerp(control, to, progress);

        push_outline_point(outline, point_lerp(first, second, progress));
    }
}

fn push_outline_point(outline: &mut Vec<Point>, point: Point) {
    if outline
        .last()
        .map_or(true, |previous| distance(*previous, point) > 0.0001)
    {
        outline.push(point);
    }
}

fn center_points_on_bounds(points: &[Point]) -> Vec<Point> {
    let center = points_bounds_center(points);

    points
        .iter()
        .map(|point| point_sub(*point, center))
        .collect()
}

fn points_bounds_center(points: &[Point]) -> Point {
    if points.is_empty() {
        return Point::ORIGIN;
    }

    let (min, max) = points_bounds(points);

    Point::new((min.x + max.x) / 2.0, (min.y + max.y) / 2.0)
}

#[cfg(test)]
fn points_bounds_size(points: &[Point]) -> (f32, f32) {
    let (min, max) = points_bounds(points);

    (max.x - min.x, max.y - min.y)
}

fn points_bounds(points: &[Point]) -> (Point, Point) {
    if points.is_empty() {
        return (Point::ORIGIN, Point::ORIGIN);
    }

    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for point in points {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
    }

    (Point::new(min_x, min_y), Point::new(max_x, max_y))
}

fn distance(from: Point, to: Point) -> f32 {
    (to.x - from.x).hypot(to.y - from.y)
}

fn squared_distance(from: Point, to: Point) -> f32 {
    let x = to.x - from.x;
    let y = to.y - from.y;

    x * x + y * y
}

fn point_add(from: Point, to: Point) -> Point {
    Point::new(from.x + to.x, from.y + to.y)
}

fn point_sub(from: Point, to: Point) -> Point {
    Point::new(from.x - to.x, from.y - to.y)
}

fn point_scale(point: Point, scale: f32) -> Point {
    Point::new(point.x * scale, point.y * scale)
}

fn point_lerp(from: Point, to: Point, progress: f32) -> Point {
    Point::new(
        from.x + (to.x - from.x) * progress,
        from.y + (to.y - from.y) * progress,
    )
}

fn point_length(point: Point) -> f32 {
    point.x.hypot(point.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.001,
            "actual={actual} expected={expected}"
        );
    }

    #[test]
    fn indeterminate_keyframes_match_material3_head_tail_timing() {
        let start = indeterminate_bars(0.0);
        assert_close(start[0].head, 0.0);
        assert_close(start[0].tail, 0.0);
        assert_close(start[1].head, 0.0);
        assert_close(start[1].tail, 0.0);

        let first_head_done = indeterminate_bars(1000.0 / 1750.0);
        assert_close(first_head_done[0].head, 1.0);
        assert!(first_head_done[0].tail > 0.0);

        let second_head_started = indeterminate_bars(700.0 / 1750.0);
        assert!(second_head_started[1].head > 0.0);
        assert_close(second_head_started[1].tail, 0.0);
    }

    #[test]
    fn determinate_wavy_amplitude_flattens_near_edges() {
        assert_eq!(determinate_wave_amplitude(0.0), 0.0);
        assert_eq!(determinate_wave_amplitude(0.1), 0.0);
        assert_eq!(determinate_wave_amplitude(0.5), 1.0);
        assert_eq!(determinate_wave_amplitude(0.95), 0.0);
    }

    #[test]
    fn four_color_indicator_uses_material_color_windows() {
        let primary = Color::from_rgb(1.0, 0.0, 0.0);
        let primary_container = Color::from_rgb(0.0, 1.0, 0.0);
        let tertiary = Color::from_rgb(0.0, 0.0, 1.0);
        let tertiary_container = Color::from_rgb(1.0, 1.0, 0.0);

        assert_eq!(
            four_color_indicator(
                primary,
                primary_container,
                tertiary,
                tertiary_container,
                0.10
            ),
            primary
        );
        assert_eq!(
            four_color_indicator(
                primary,
                primary_container,
                tertiary,
                tertiary_container,
                0.30
            ),
            primary_container
        );
        assert_eq!(
            four_color_indicator(
                primary,
                primary_container,
                tertiary,
                tertiary_container,
                0.55
            ),
            tertiary
        );
        assert_eq!(
            four_color_indicator(
                primary,
                primary_container,
                tertiary,
                tertiary_container,
                0.80
            ),
            tertiary_container
        );
    }

    #[test]
    fn loading_indicator_uses_material_shape_sequence_vertices_and_rounding() {
        assert_eq!(material_loading_vertices(0).len(), 20);
        assert!(
            material_loading_vertices(0)
                .iter()
                .all(|vertex| vertex.rounding > 0.0)
        );
        assert_eq!(material_loading_vertices(1).len(), 18);
        assert!(
            material_loading_vertices(1)
                .iter()
                .all(|vertex| vertex.rounding > 0.0)
        );
        assert_eq!(material_loading_vertices(2).len(), 5);
        assert!(
            material_loading_vertices(2)
                .iter()
                .all(|vertex| vertex.rounding > 0.0)
        );
        assert_eq!(material_loading_vertices(3).len(), 10);
        assert!(
            material_loading_vertices(3)
                .iter()
                .any(|vertex| vertex.rounding == 0.0)
        );
        assert!(
            material_loading_vertices(3)
                .iter()
                .any(|vertex| vertex.rounding > 0.0)
        );
        assert_eq!(material_loading_vertices(4).len(), 16);
        assert!(
            material_loading_vertices(4)
                .iter()
                .all(|vertex| vertex.rounding > 0.0)
        );
        assert_eq!(material_loading_vertices(5).len(), 8);
        assert!(
            material_loading_vertices(5)
                .iter()
                .all(|vertex| vertex.rounding > 0.0)
        );
        assert_eq!(material_loading_vertices(6).len(), CIRCLE_SAMPLE_COUNT);
    }

    #[test]
    fn loading_shape_sequence_wraps_after_seven_shapes() {
        let first = material_loading_vertices(0);
        let repeated = material_loading_vertices(
            tokens::component::loading_indicator::INDETERMINATE_SHAPE_COUNT,
        );

        assert_eq!(first, repeated);
    }

    #[test]
    fn determinate_loading_shape_sequence_uses_circle_to_soft_burst() {
        assert_eq!(determinate_loading_vertices(0).len(), CIRCLE_SAMPLE_COUNT);
        assert_eq!(
            determinate_loading_vertices(1),
            material_loading_vertices(0)
        );
    }

    #[test]
    fn loading_shape_outlines_are_densely_rounded() {
        for shape_index in 0..tokens::component::loading_indicator::INDETERMINATE_SHAPE_COUNT {
            let outline = material_loading_outline(shape_index);

            assert!(
                outline.len() >= 64,
                "shape {shape_index} outline is undersampled: {}",
                outline.len()
            );
            assert!(
                max_turn_angle(&outline) < 0.7,
                "shape {shape_index} has a hard outline turn: {}",
                max_turn_angle(&outline)
            );
        }
    }

    #[test]
    fn loading_morphs_do_not_create_spike_turns() {
        for shape_index in 0..tokens::component::loading_indicator::INDETERMINATE_SHAPE_COUNT {
            let from = material_loading_outline(shape_index);
            let to = material_loading_outline(
                (shape_index + 1) % tokens::component::loading_indicator::INDETERMINATE_SHAPE_COUNT,
            );

            for progress in [0.0, 0.25, 0.5, 0.75, 1.0] {
                let points = morphed_loading_points(&from, &to, progress);
                let max_turn = max_turn_angle(&points);

                assert!(
                    max_turn < 0.7,
                    "shape {shape_index} morph {progress} has a spike turn: {max_turn}"
                );
            }
        }
    }

    #[test]
    fn loading_morphs_are_centered_before_rotation() {
        for shape_index in 0..tokens::component::loading_indicator::INDETERMINATE_SHAPE_COUNT {
            let from = material_loading_outline(shape_index);
            let to = material_loading_outline(
                (shape_index + 1) % tokens::component::loading_indicator::INDETERMINATE_SHAPE_COUNT,
            );

            for progress in [0.0, 0.125, 0.25, 0.5, 0.75, 0.875, 1.0] {
                let points = morphed_loading_points(&from, &to, progress);
                let center = points_bounds_center(&points);

                assert!(
                    center.x.abs() < 0.001 && center.y.abs() < 0.001,
                    "shape {shape_index} morph {progress} is off-center: {center:?}"
                );
            }
        }
    }

    #[test]
    fn loading_morph_endpoints_preserve_source_bounds() {
        for shape_index in 0..tokens::component::loading_indicator::INDETERMINATE_SHAPE_COUNT {
            let from = material_loading_outline(shape_index);
            let to = material_loading_outline(
                (shape_index + 1) % tokens::component::loading_indicator::INDETERMINATE_SHAPE_COUNT,
            );
            let centered_from = center_points_on_bounds(&from);
            let centered_to = center_points_on_bounds(&to);
            let start_points = morphed_loading_points(&from, &to, 0.0);
            let end_points = morphed_loading_points(&from, &to, 1.0);

            assert_bounds_close(
                points_bounds_size(&start_points),
                points_bounds_size(&centered_from),
                shape_index,
                0.0,
            );
            assert_bounds_close(
                points_bounds_size(&end_points),
                points_bounds_size(&centered_to),
                shape_index,
                1.0,
            );
        }
    }

    #[test]
    fn loading_morph_spring_reaches_target_before_interval_end() {
        assert_close(loading_spring_progress(0.0), 0.0);
        assert!(loading_spring_progress(0.5) > 0.8);
        assert!(loading_spring_progress(1.0) > 0.99);
    }

    #[test]
    fn loading_shape_scale_accounts_for_rotation_bounds() {
        let scale = loading_shape_scale();

        assert!(scale > 0.0);
        assert!(scale < tokens::component::loading_indicator::ACTIVE_INDICATOR_SCALE);
    }

    fn max_turn_angle(points: &[Point]) -> f32 {
        let mut max_angle = 0.0_f32;

        for index in 0..points.len() {
            let previous = points[(index + points.len() - 1) % points.len()];
            let current = points[index];
            let next = points[(index + 1) % points.len()];
            let incoming = point_sub(current, previous);
            let outgoing = point_sub(next, current);
            let incoming_length = point_length(incoming);
            let outgoing_length = point_length(outgoing);

            if incoming_length <= f32::EPSILON || outgoing_length <= f32::EPSILON {
                continue;
            }

            let dot = (incoming.x * outgoing.x + incoming.y * outgoing.y)
                / (incoming_length * outgoing_length);
            let angle = dot.clamp(-1.0, 1.0).acos();

            max_angle = max_angle.max(angle);
        }

        max_angle
    }

    fn assert_bounds_close(
        actual: (f32, f32),
        expected: (f32, f32),
        shape_index: usize,
        progress: f32,
    ) {
        assert!(
            (actual.0 - expected.0).abs() < 0.03 && (actual.1 - expected.1).abs() < 0.03,
            "shape {shape_index} morph {progress} changed bounds: actual={actual:?} expected={expected:?}"
        );
    }
}
