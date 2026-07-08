use iced_widget::canvas;
use iced_widget::core::time::{Duration, Instant};
use iced_widget::core::{Color, Point, Rectangle, Size, Vector, border};
use iced_widget::graphics::geometry;
use iced_widget::renderer::wgpu::{self, primitive};

use crate::tokens;
use crate::utils::state_layer;

use super::support::{duration_ms, lerp};

pub(super) const RIPPLE_CLIP_MIN_SAMPLES: usize = 24;
pub(super) const RIPPLE_CLIP_MAX_SAMPLES: usize = 96;
// Mirrors AOSP RippleAnimationSession.MAX_NOISE_PHASE = duration / 214.
const NOISE_PHASE_DURATION_DIVISOR: f32 = 214.0;
const RIPPLE_SHADER_FLOATS: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RippleStart {
    Additive,
    Replace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RippleTiming {
    Common,
    Patterned,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub(super) enum RippleStyle {
    Solid,
    Patterned { effect_color: Color },
}

impl RippleStyle {
    pub(super) fn material_patterned() -> Self {
        Self::Patterned {
            effect_color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: tokens::state::RIPPLE_PATTERN_DEFAULT_EFFECT_ALPHA,
            },
        }
    }

    fn timing(self) -> RippleTiming {
        match self {
            Self::Solid => RippleTiming::Common,
            Self::Patterned { .. } => RippleTiming::Patterned,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct RippleConfig {
    pub(super) bounded: bool,
    pub(super) radius: Option<f32>,
    pub(super) clip_radius: border::Radius,
    pub(super) press_opacity: f32,
    pub(super) style: RippleStyle,
}

impl RippleConfig {
    pub(super) fn bounded(clip_radius: border::Radius) -> Self {
        Self {
            bounded: true,
            radius: None,
            clip_radius,
            press_opacity: tokens::state::PRESSED_STATE_LAYER_OPACITY,
            style: RippleStyle::material_patterned(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct PressRippleState {
    active_ripple: Option<PressRipple>,
    exiting_ripples: Vec<PressRipple>,
}

impl PressRippleState {
    pub(super) fn press(
        &mut self,
        origin: Point,
        now: Instant,
        start: RippleStart,
        style: RippleStyle,
    ) {
        match start {
            RippleStart::Additive => {
                if let Some(mut ripple) = self.active_ripple.take() {
                    ripple.exit(now);
                    self.push_exiting(ripple);
                }
            }
            RippleStart::Replace => {
                self.clear();
            }
        }

        self.active_ripple = Some(PressRipple::with_timing(origin, now, style.timing()));
    }

    pub(super) fn release(&mut self, now: Instant) {
        if let Some(mut ripple) = self.active_ripple.take() {
            ripple.exit(now);
            self.push_exiting(ripple);
        }
    }

    pub(super) fn release_replacing(&mut self, now: Instant) {
        self.exiting_ripples.clear();

        if let Some(mut ripple) = self.active_ripple.take() {
            ripple.exit(now);
            self.exiting_ripples.push(ripple);
        }
    }

    pub(super) fn clear(&mut self) {
        self.active_ripple = None;
        self.exiting_ripples.clear();
    }

    pub(super) fn prune(&mut self, now: Instant) {
        self.exiting_ripples
            .retain(|ripple| !ripple.has_finished_exit(now));
    }

    pub(super) fn has_visible_ripples(&self, now: Instant) -> bool {
        self.ripple_opacity(now) > 0.0
    }

    pub(super) fn ripple_opacity(&self, now: Instant) -> f32 {
        self.active_ripple
            .map_or(0.0, |ripple| ripple.opacity(now))
            .max(
                self.exiting_ripples
                    .iter()
                    .map(|ripple| ripple.opacity(now))
                    .fold(0.0, f32::max),
            )
    }

    #[cfg(test)]
    pub(super) fn has_active_ripple(&self) -> bool {
        self.active_ripple.is_some()
    }

    #[cfg(test)]
    pub(super) fn exiting_ripple_count(&self) -> usize {
        self.exiting_ripples.len()
    }

    fn push_exiting(&mut self, ripple: PressRipple) {
        if self.exiting_ripples.len() >= tokens::state::RIPPLE_MAX_RIPPLES {
            let _ = self.exiting_ripples.remove(0);
        }

        self.exiting_ripples.push(ripple);
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct PressRipple {
    origin: Point,
    started_at: Instant,
    exit_started_at: Option<Instant>,
    pub(super) exit_delay: Duration,
    timing: RippleTiming,
}

impl PressRipple {
    #[cfg(test)]
    pub(super) fn new(origin: Point, started_at: Instant) -> Self {
        Self::with_timing(origin, started_at, RippleTiming::Common)
    }

    fn with_timing(origin: Point, started_at: Instant, timing: RippleTiming) -> Self {
        Self {
            origin,
            started_at,
            exit_started_at: None,
            exit_delay: Duration::ZERO,
            timing,
        }
    }

    pub(super) fn exit(&mut self, now: Instant) {
        let hold = self.timing.minimum_visible_duration();
        let elapsed = now.duration_since(self.started_at);

        self.exit_started_at = Some(now);
        self.exit_delay = hold.saturating_sub(elapsed);
    }

    #[cfg(test)]
    pub(super) fn circle(self, size: Size, now: Instant) -> RippleCircle {
        self.circle_with(size, now, true, None)
    }

    fn circle_with(
        self,
        size: Size,
        now: Instant,
        bounded: bool,
        explicit_radius: Option<f32>,
    ) -> RippleCircle {
        let target_radius = explicit_radius.unwrap_or_else(|| ripple_end_radius(bounded, size));
        let start_radius = get_ripple_start_radius(size);
        let origin = if bounded {
            clamped_ripple_origin(self.origin, size, target_radius, start_radius)
        } else {
            Point::new(size.width / 2.0, size.height / 2.0)
        };
        let radius_progress = timed_progress(
            self.started_at,
            now,
            duration_ms(tokens::state::RIPPLE_RADIUS_DURATION_MS),
            tokens::motion::EASING_LEGACY,
        );
        let origin_progress = timed_progress(
            self.started_at,
            now,
            duration_ms(tokens::state::RIPPLE_ORIGIN_DURATION_MS),
            tokens::motion::EASING_LEGACY,
        );
        let center = Point::new(size.width / 2.0, size.height / 2.0);

        RippleCircle {
            center: Point::new(
                lerp(origin.x, center.x, origin_progress),
                lerp(origin.y, center.y, origin_progress),
            ),
            radius: lerp(start_radius, target_radius, radius_progress),
            target_radius,
        }
    }

    pub(super) fn opacity(self, now: Instant) -> f32 {
        match self.timing {
            RippleTiming::Common => self.common_opacity(now),
            RippleTiming::Patterned => self.patterned_opacity(now),
        }
    }

    fn common_opacity(self, now: Instant) -> f32 {
        let enter = timed_progress(
            self.started_at,
            now,
            duration_ms(tokens::state::RIPPLE_FADE_IN_DURATION_MS),
            tokens::motion::EASING_LINEAR,
        );

        enter * self.exit_opacity(now, duration_ms(tokens::state::RIPPLE_FADE_OUT_DURATION_MS))
    }

    fn patterned_opacity(self, now: Instant) -> f32 {
        let progress = self.patterned_progress(now);
        let fade_in = sub_progress(0.0, tokens::state::RIPPLE_PATTERN_FADE_IN_END, progress);
        let fade_out = sub_progress(tokens::state::RIPPLE_PATTERN_FADE_OUT_START, 1.0, progress);

        fade_in.min(1.0 - fade_out).clamp(0.0, 1.0)
    }

    fn exit_opacity(self, now: Instant, fade_out_duration: Duration) -> f32 {
        self.exit_started_at
            .map(|exit_started_at| {
                let elapsed = now.duration_since(exit_started_at);

                if elapsed <= self.exit_delay {
                    1.0
                } else {
                    let fade = elapsed - self.exit_delay;
                    1.0 - (fade.as_secs_f32() / fade_out_duration.as_secs_f32()).clamp(0.0, 1.0)
                }
            })
            .unwrap_or(1.0)
    }

    pub(super) fn has_finished_exit(self, now: Instant) -> bool {
        self.exit_started_at.is_some_and(|exit_started_at| {
            now.duration_since(exit_started_at) >= self.exit_delay + self.timing.exit_duration()
        })
    }

    fn patterned_progress(self, now: Instant) -> f32 {
        let enter_duration = duration_ms(tokens::state::RIPPLE_PATTERN_ENTER_DURATION_MS);
        let exit_duration = duration_ms(tokens::state::RIPPLE_PATTERN_EXIT_DURATION_MS);

        if let Some(exit_started_at) = self.exit_started_at {
            let elapsed = now.duration_since(exit_started_at);

            if elapsed <= self.exit_delay {
                self.patterned_enter_progress(now, enter_duration)
            } else {
                let exit_elapsed = elapsed - self.exit_delay;
                lerp(
                    0.5,
                    1.0,
                    (exit_elapsed.as_secs_f32() / exit_duration.as_secs_f32()).clamp(0.0, 1.0),
                )
            }
        } else {
            self.patterned_enter_progress(now, enter_duration)
        }
    }

    fn patterned_enter_progress(self, now: Instant, enter_duration: Duration) -> f32 {
        let progress = (now.duration_since(self.started_at).as_secs_f32()
            / enter_duration.as_secs_f32())
        .clamp(0.0, 1.0);

        lerp(0.0, 0.5, tokens::motion::EASING_LEGACY.transform(progress))
    }
}

impl RippleTiming {
    fn minimum_visible_duration(self) -> Duration {
        match self {
            Self::Common => duration_ms(tokens::state::RIPPLE_OPACITY_HOLD_DURATION_MS),
            Self::Patterned => duration_ms(tokens::state::RIPPLE_PATTERN_ENTER_DURATION_MS),
        }
    }

    fn exit_duration(self) -> Duration {
        match self {
            Self::Common => duration_ms(tokens::state::RIPPLE_FADE_OUT_DURATION_MS),
            Self::Patterned => duration_ms(tokens::state::RIPPLE_PATTERN_EXIT_DURATION_MS),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct RippleCircle {
    pub(super) center: Point,
    pub(super) radius: f32,
    target_radius: f32,
}

pub(super) fn draw_ripples<Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    state: &PressRippleState,
    color: Color,
    config: RippleConfig,
    now: Instant,
) where
    Renderer: geometry::Renderer + primitive::Renderer,
{
    if !state.has_visible_ripples(now) {
        return;
    }

    let ripple_color = state_layer(color, config.press_opacity);

    match config.style {
        RippleStyle::Solid => {
            draw_solid_ripples(renderer, bounds, state, ripple_color, config, now);
        }
        RippleStyle::Patterned { effect_color } => {
            draw_patterned_ripples(
                renderer,
                bounds,
                state,
                ripple_color,
                effect_color,
                config,
                now,
            );
        }
    }
}

fn draw_solid_ripples<Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    state: &PressRippleState,
    color: Color,
    config: RippleConfig,
    now: Instant,
) where
    Renderer: geometry::Renderer,
{
    let mut frame = canvas::Frame::new(renderer, bounds.size());

    if let Some(ripple) = state.active_ripple {
        fill_solid_ripple(&mut frame, ripple, bounds.size(), color, config, now);
    }

    for ripple in &state.exiting_ripples {
        fill_solid_ripple(&mut frame, *ripple, bounds.size(), color, config, now);
    }

    let geometry = frame.into_geometry();

    renderer.with_layer(bounds, |renderer| {
        renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
            renderer.draw_geometry(geometry);
        });
    });
}

fn draw_patterned_ripples<Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    state: &PressRippleState,
    color: Color,
    effect_color: Color,
    config: RippleConfig,
    now: Instant,
) where
    Renderer: primitive::Renderer,
{
    if let Some(ripple) = state.active_ripple {
        draw_patterned_ripple(renderer, ripple, bounds, color, effect_color, config, now);
    }

    for ripple in &state.exiting_ripples {
        draw_patterned_ripple(renderer, *ripple, bounds, color, effect_color, config, now);
    }
}

fn fill_solid_ripple<Renderer>(
    frame: &mut canvas::Frame<Renderer>,
    ripple: PressRipple,
    size: Size,
    mut color: Color,
    config: RippleConfig,
    now: Instant,
) where
    Renderer: geometry::Renderer,
{
    let opacity = ripple.common_opacity(now);

    if opacity <= 0.0 {
        return;
    }

    let circle = ripple.circle_with(size, now, config.bounded, config.radius);

    if circle.radius <= 0.0 {
        return;
    }

    color.a *= opacity;

    let path = ripple_path(size, config, circle);

    frame.fill(&path, color);
}

fn draw_patterned_ripple<Renderer>(
    renderer: &mut Renderer,
    ripple: PressRipple,
    bounds: Rectangle,
    color: Color,
    effect_color: Color,
    config: RippleConfig,
    now: Instant,
) where
    Renderer: primitive::Renderer,
{
    let size = bounds.size();
    let progress = ripple.patterned_progress(now);
    let fade_in = sub_progress(0.0, tokens::state::RIPPLE_PATTERN_FADE_IN_END, progress);
    let fade_out_noise = sub_progress(
        tokens::state::RIPPLE_PATTERN_FADE_OUT_START,
        tokens::state::RIPPLE_PATTERN_FADE_OUT_NOISE_END,
        progress,
    );
    let fade_out_ripple = sub_progress(tokens::state::RIPPLE_PATTERN_FADE_OUT_START, 1.0, progress);
    let alpha = fade_in.min(1.0 - fade_out_noise).clamp(0.0, 1.0);
    let fade = fade_in.min(1.0 - fade_out_ripple).clamp(0.0, 1.0);

    if fade <= 0.0 && alpha <= 0.0 {
        return;
    }

    let target_radius = config
        .radius
        .unwrap_or_else(|| ripple_end_radius(config.bounded, size));
    let max_radius = target_radius * tokens::state::RIPPLE_PATTERN_RADIUS_SCALE;
    let origin = Point::new(size.width / 2.0, size.height / 2.0);
    let touch = if config.bounded {
        ripple.origin
    } else {
        origin
    };
    let phase = noise_phase(ripple, now);

    renderer.draw_primitive(
        bounds,
        RippleShaderPrimitive::new(
            RippleShaderUniforms {
                size,
                touch,
                origin,
                progress,
                max_radius,
                noise_phase: phase,
                color,
                sparkle_color: effect_color,
                clip_radius: config.clip_radius,
                bounded: config.bounded,
            },
            bounds,
        ),
    );
}

#[derive(Debug, Clone, Copy)]
struct RippleShaderUniforms {
    size: Size,
    touch: Point,
    origin: Point,
    progress: f32,
    max_radius: f32,
    noise_phase: NoisePhase,
    color: Color,
    sparkle_color: Color,
    clip_radius: border::Radius,
    bounded: bool,
}

impl RippleShaderUniforms {
    fn key(self, bounds: Rectangle) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        for value in self.floats(bounds, 1.0) {
            std::hash::Hash::hash(&value.to_bits(), &mut hasher);
        }

        std::hash::Hasher::finish(&hasher)
    }

    fn bytes(self, bounds: Rectangle, scale_factor: f32) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(RIPPLE_SHADER_FLOATS * std::mem::size_of::<f32>());

        for value in self.floats(bounds, scale_factor) {
            bytes.extend_from_slice(&value.to_ne_bytes());
        }

        bytes
    }

    fn floats(self, bounds: Rectangle, scale_factor: f32) -> [f32; RIPPLE_SHADER_FLOATS] {
        let phase = self.noise_phase;
        let turbulence = TurbulenceUniforms::new(phase.turbulence);

        [
            bounds.x * scale_factor,
            bounds.y * scale_factor,
            scale_factor,
            if self.bounded { 1.0 } else { 0.0 },
            self.size.width,
            self.size.height,
            tokens::state::RIPPLE_PATTERN_NOISE_DENSITY_SCALE
                / (self.size.width * scale_factor).max(1.0),
            tokens::state::RIPPLE_PATTERN_NOISE_DENSITY_SCALE
                / (self.size.height * scale_factor).max(1.0),
            self.touch.x,
            self.touch.y,
            self.origin.x,
            self.origin.y,
            self.progress,
            self.max_radius,
            phase.sparkle,
            phase.turbulence,
            self.color.r,
            self.color.g,
            self.color.b,
            self.color.a,
            self.sparkle_color.r,
            self.sparkle_color.g,
            self.sparkle_color.b,
            self.sparkle_color.a,
            turbulence.circle1.x,
            turbulence.circle1.y,
            turbulence.circle2.x,
            turbulence.circle2.y,
            turbulence.circle3.x,
            turbulence.circle3.y,
            turbulence.rotation1.x,
            turbulence.rotation1.y,
            turbulence.rotation2.x,
            turbulence.rotation2.y,
            turbulence.rotation3.x,
            turbulence.rotation3.y,
            self.clip_radius.top_left,
            self.clip_radius.top_right,
            self.clip_radius.bottom_right,
            self.clip_radius.bottom_left,
        ]
    }
}

#[derive(Debug, Clone, Copy)]
struct RippleShaderPrimitive {
    uniforms: RippleShaderUniforms,
    key: u64,
}

impl RippleShaderPrimitive {
    fn new(uniforms: RippleShaderUniforms, bounds: Rectangle) -> Self {
        Self {
            uniforms,
            key: uniforms.key(bounds),
        }
    }
}

impl primitive::Primitive for RippleShaderPrimitive {
    type Pipeline = RippleShaderPipeline;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &wgpu::wgpu::Device,
        queue: &wgpu::wgpu::Queue,
        bounds: &Rectangle,
        viewport: &iced_widget::graphics::Viewport,
    ) {
        pipeline.prepare(
            self.key,
            device,
            queue,
            self.uniforms.bytes(*bounds, viewport.scale_factor()),
        );
    }

    fn draw(
        &self,
        pipeline: &Self::Pipeline,
        render_pass: &mut wgpu::wgpu::RenderPass<'_>,
    ) -> bool {
        pipeline.draw(self.key, render_pass)
    }
}

struct RippleShaderPipeline {
    pipeline: wgpu::wgpu::RenderPipeline,
    bind_group_layout: wgpu::wgpu::BindGroupLayout,
    prepared: std::collections::HashMap<u64, PreparedRippleShader>,
}

struct PreparedRippleShader {
    _buffer: wgpu::wgpu::Buffer,
    bind_group: wgpu::wgpu::BindGroup,
}

impl primitive::Pipeline for RippleShaderPipeline {
    fn new(
        device: &wgpu::wgpu::Device,
        _queue: &wgpu::wgpu::Queue,
        format: wgpu::wgpu::TextureFormat,
    ) -> Self {
        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::wgpu::BindGroupLayoutDescriptor {
                label: Some("material_ui_rs.ripple_shader.bind_group_layout"),
                entries: &[wgpu::wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::wgpu::BindingType::Buffer {
                        ty: wgpu::wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::wgpu::PipelineLayoutDescriptor {
                label: Some("material_ui_rs.ripple_shader.pipeline_layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader = device.create_shader_module(wgpu::wgpu::ShaderModuleDescriptor {
            label: Some("material_ui_rs.ripple_shader.shader"),
            source: wgpu::wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "ripple_shader.wgsl"
            ))),
        });

        let pipeline = device.create_render_pipeline(&wgpu::wgpu::RenderPipelineDescriptor {
            label: Some("material_ui_rs.ripple_shader.pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::wgpu::PrimitiveState {
                topology: wgpu::wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::wgpu::FrontFace::Cw,
                ..wgpu::wgpu::PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: wgpu::wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            bind_group_layout,
            prepared: std::collections::HashMap::new(),
        }
    }

    fn trim(&mut self) {
        self.prepared.clear();
    }
}

impl RippleShaderPipeline {
    fn prepare(
        &mut self,
        key: u64,
        device: &wgpu::wgpu::Device,
        queue: &wgpu::wgpu::Queue,
        bytes: Vec<u8>,
    ) {
        let buffer = device.create_buffer(&wgpu::wgpu::BufferDescriptor {
            label: Some("material_ui_rs.ripple_shader.uniform_buffer"),
            size: bytes.len() as u64,
            usage: wgpu::wgpu::BufferUsages::UNIFORM | wgpu::wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(&buffer, 0, &bytes);

        let bind_group = device.create_bind_group(&wgpu::wgpu::BindGroupDescriptor {
            label: Some("material_ui_rs.ripple_shader.bind_group"),
            layout: &self.bind_group_layout,
            entries: &[wgpu::wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        let _ = self.prepared.insert(
            key,
            PreparedRippleShader {
                _buffer: buffer,
                bind_group,
            },
        );
    }

    fn draw(&self, key: u64, render_pass: &mut wgpu::wgpu::RenderPass<'_>) -> bool {
        let Some(prepared) = self.prepared.get(&key) else {
            return false;
        };

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &prepared.bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        true
    }
}

fn ripple_path(size: Size, config: RippleConfig, circle: RippleCircle) -> canvas::Path {
    if !config.bounded {
        return canvas::Path::circle(circle.center, circle.radius);
    }

    bounded_ripple_path(size, config.clip_radius, circle)
}

pub(super) fn bounded_ripple_path(
    size: Size,
    clip_radius: border::Radius,
    circle: RippleCircle,
) -> canvas::Path {
    if circle.radius >= circle.target_radius - 0.5 {
        return canvas::Path::rounded_rectangle(Point::ORIGIN, size, clip_radius);
    }

    clipped_circle_path(size, clip_radius, circle)
        .unwrap_or_else(|| canvas::Path::circle(circle.center, circle.radius))
}

fn clipped_circle_path(
    size: Size,
    clip_radius: border::Radius,
    circle: RippleCircle,
) -> Option<canvas::Path> {
    let top = (circle.center.y - circle.radius).max(0.0);
    let bottom = (circle.center.y + circle.radius).min(size.height);

    if bottom <= top {
        return None;
    }

    let sample_count = ripple_clip_sample_count(circle.radius);
    let step = (bottom - top) / (sample_count.saturating_sub(1) as f32);
    let mut left_edge = Vec::with_capacity(sample_count);
    let mut right_edge = Vec::with_capacity(sample_count);

    for index in 0..sample_count {
        let y = if index + 1 == sample_count {
            bottom
        } else {
            top + step * index as f32
        };

        let Some((circle_left, circle_right)) = circle_span_at_y(circle, y) else {
            continue;
        };
        let Some((clip_left, clip_right)) = rounded_rect_span_at_y(size, clip_radius, y) else {
            continue;
        };

        let left = circle_left.max(clip_left);
        let right = circle_right.min(clip_right);

        if left <= right {
            left_edge.push(Point::new(left, y));
            right_edge.push(Point::new(right, y));
        }
    }

    if left_edge.len() < 2 || right_edge.len() < 2 {
        return None;
    }

    Some(canvas::Path::new(|path| {
        path.move_to(left_edge[0]);

        for point in left_edge.iter().skip(1) {
            path.line_to(*point);
        }

        for point in right_edge.iter().rev() {
            path.line_to(*point);
        }

        path.close();
    }))
}

pub(super) fn ripple_clip_sample_count(radius: f32) -> usize {
    ((radius * std::f32::consts::TAU).ceil() as usize)
        .clamp(RIPPLE_CLIP_MIN_SAMPLES, RIPPLE_CLIP_MAX_SAMPLES)
}

fn circle_span_at_y(circle: RippleCircle, y: f32) -> Option<(f32, f32)> {
    let dy = y - circle.center.y;
    let distance_to_edge_squared = circle.radius * circle.radius - dy * dy;

    if distance_to_edge_squared < 0.0 {
        return None;
    }

    let dx = distance_to_edge_squared.sqrt();

    Some((circle.center.x - dx, circle.center.x + dx))
}

pub(super) fn rounded_rect_span_at_y(
    size: Size,
    radius: border::Radius,
    y: f32,
) -> Option<(f32, f32)> {
    if y < 0.0 || y > size.height {
        return None;
    }

    let [top_left, top_right, bottom_right, bottom_left] = normalized_corner_radii(size, radius);
    let mut left: f32 = 0.0;
    let mut right = size.width;

    if top_left > 0.0 && y < top_left {
        left = left.max(corner_left_bound(top_left, y, top_left));
    }

    if bottom_left > 0.0 && y > size.height - bottom_left {
        left = left.max(corner_left_bound(bottom_left, y, size.height - bottom_left));
    }

    if top_right > 0.0 && y < top_right {
        right = right.min(corner_right_bound(size.width, top_right, y, top_right));
    }

    if bottom_right > 0.0 && y > size.height - bottom_right {
        right = right.min(corner_right_bound(
            size.width,
            bottom_right,
            y,
            size.height - bottom_right,
        ));
    }

    (left <= right).then_some((left, right))
}

fn normalized_corner_radii(size: Size, radius: border::Radius) -> [f32; 4] {
    let max_radius = size.width.min(size.height) / 2.0;
    let [top_left, top_right, bottom_right, bottom_left] = radius.into();

    [
        top_left.min(max_radius),
        top_right.min(max_radius),
        bottom_right.min(max_radius),
        bottom_left.min(max_radius),
    ]
}

fn corner_left_bound(radius: f32, y: f32, center_y: f32) -> f32 {
    radius - circle_axis_delta(radius, y - center_y)
}

fn corner_right_bound(width: f32, radius: f32, y: f32, center_y: f32) -> f32 {
    width - radius + circle_axis_delta(radius, y - center_y)
}

fn circle_axis_delta(radius: f32, offset: f32) -> f32 {
    (radius * radius - offset * offset).max(0.0).sqrt()
}

pub(super) fn timed_progress(
    started_at: Instant,
    now: Instant,
    duration: Duration,
    easing: tokens::motion::CubicBezier,
) -> f32 {
    if duration.is_zero() {
        return 1.0;
    }

    let progress =
        (now.duration_since(started_at).as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0);

    easing.transform(progress)
}

#[cfg(test)]
pub(super) fn ripple_target_radius(size: Size) -> f32 {
    ripple_end_radius(true, size)
}

#[cfg(test)]
pub(super) fn unbounded_ripple_target_radius(size: Size) -> f32 {
    ripple_end_radius(false, size)
}

fn ripple_end_radius(bounded: bool, size: Size) -> f32 {
    let half_width = size.width / 2.0;
    let half_height = size.height / 2.0;
    let radius_covering_bounds = (half_width * half_width + half_height * half_height).sqrt();

    if bounded {
        radius_covering_bounds + tokens::state::RIPPLE_BOUNDED_EXTRA_RADIUS
    } else {
        radius_covering_bounds
    }
}

pub(super) fn get_ripple_start_radius(size: Size) -> f32 {
    size.width.max(size.height) * tokens::state::RIPPLE_START_RADIUS_FACTOR
}

pub(super) fn clamped_ripple_origin(
    origin: Point,
    size: Size,
    target_radius: f32,
    start_radius: f32,
) -> Point {
    let center = Point::new(size.width / 2.0, size.height / 2.0);
    let dx = origin.x - center.x;
    let dy = origin.y - center.y;
    let radius = (target_radius - start_radius).max(0.0);
    let distance_squared = dx * dx + dy * dy;

    if radius > 0.0 && distance_squared > radius * radius {
        let angle = dy.atan2(dx);

        Point::new(
            center.x + angle.cos() * radius,
            center.y + angle.sin() * radius,
        )
    } else {
        origin
    }
}

fn sub_progress(start: f32, end: f32, progress: f32) -> f32 {
    let clamped = progress.clamp(start, end);

    if (end - start).abs() <= f32::EPSILON {
        1.0
    } else {
        (clamped - start) / (end - start)
    }
}

#[derive(Debug, Clone, Copy)]
struct NoisePhase {
    sparkle: f32,
    turbulence: f32,
}

fn noise_phase(ripple: PressRipple, now: Instant) -> NoisePhase {
    let elapsed_ms = now.duration_since(ripple.started_at).as_secs_f32() * 1000.0;
    let max_phase = tokens::state::RIPPLE_PATTERN_NOISE_ANIMATION_DURATION_MS as f32
        / NOISE_PHASE_DURATION_DIVISOR;
    let turbulence = (elapsed_ms / NOISE_PHASE_DURATION_DIVISOR).clamp(0.0, max_phase);

    NoisePhase {
        sparkle: turbulence * 0.001,
        turbulence,
    }
}

#[cfg(test)]
pub(super) fn ripple_noise_phases(ripple: PressRipple, now: Instant) -> (f32, f32) {
    let phase = noise_phase(ripple, now);

    (phase.sparkle, phase.turbulence)
}

#[derive(Debug, Clone, Copy)]
struct TurbulenceUniforms {
    circle1: Point,
    circle2: Point,
    circle3: Point,
    rotation1: Point,
    rotation2: Point,
    rotation3: Point,
}

impl TurbulenceUniforms {
    fn new(phase: f32) -> Self {
        let scale: f32 = 1.5;
        let rotation_right = phase * std::f32::consts::PI * 0.0078125;
        let rotation_left = phase * std::f32::consts::PI * -0.0078125;

        Self {
            circle1: Point::new(
                scale * 0.5 + phase * 0.01 * (scale * 0.55).cos(),
                scale * 0.5 + phase * 0.01 * (scale * 0.55).sin(),
            ),
            circle2: Point::new(
                scale * 0.2 + phase * -0.0066 * (scale * 0.45).cos(),
                scale * 0.2 + phase * -0.0066 * (scale * 0.45).sin(),
            ),
            circle3: Point::new(
                scale + phase * -0.0066 * (scale * 0.35).cos(),
                scale + phase * -0.0066 * (scale * 0.35).sin(),
            ),
            rotation1: rotation(rotation_right + 1.7 * std::f32::consts::PI),
            rotation2: rotation(rotation_left + 2.0 * std::f32::consts::PI),
            rotation3: rotation(rotation_right + 2.75 * std::f32::consts::PI),
        }
    }
}

fn rotation(angle: f32) -> Point {
    Point::new(angle.cos(), angle.sin())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.001,
            "expected {expected}, got {actual}",
        );
    }

    #[test]
    fn ripple_shader_uniforms_pack_aosp_resolution_and_physical_noise_scale() {
        let uniforms = RippleShaderUniforms {
            size: Size::new(100.0, 50.0),
            touch: Point::new(12.0, 14.0),
            origin: Point::new(50.0, 25.0),
            progress: 0.4,
            max_radius: 80.0,
            noise_phase: NoisePhase {
                sparkle: 0.002,
                turbulence: 2.0,
            },
            color: Color::from_rgba(0.1, 0.2, 0.3, 0.4),
            sparkle_color: Color::from_rgba(1.0, 1.0, 1.0, 0.5),
            clip_radius: border::Radius {
                top_left: 1.0,
                top_right: 2.0,
                bottom_right: 3.0,
                bottom_left: 4.0,
            },
            bounded: true,
        };
        let floats = uniforms.floats(
            Rectangle {
                x: 10.0,
                y: 20.0,
                width: 100.0,
                height: 50.0,
            },
            2.0,
        );

        assert_eq!(floats.len(), RIPPLE_SHADER_FLOATS);
        assert_close(floats[0], 20.0);
        assert_close(floats[1], 40.0);
        assert_close(floats[2], 2.0);
        assert_close(floats[3], 1.0);
        assert_close(floats[4], 100.0);
        assert_close(floats[5], 50.0);
        assert_close(
            floats[6],
            tokens::state::RIPPLE_PATTERN_NOISE_DENSITY_SCALE / 200.0,
        );
        assert_close(
            floats[7],
            tokens::state::RIPPLE_PATTERN_NOISE_DENSITY_SCALE / 100.0,
        );
        assert_close(floats[36], 1.0);
        assert_close(floats[37], 2.0);
        assert_close(floats[38], 3.0);
        assert_close(floats[39], 4.0);
    }

    #[test]
    fn ripple_shader_source_keeps_aosp_premultiplied_sparkle_mix() {
        let source = include_str!("ripple_shader.wgsl");

        assert!(source.contains("sparkles(density_uv, noise_phase, uniforms.sparkle_color.a)"));
        assert!(source.contains("uniforms.color.rgb * wave_alpha"));
        assert!(source.contains("uniforms.sparkle_color.rgb * uniforms.sparkle_color.a"));
        assert!(source.contains("return mix(wave_color, sparkle_color, sparkle_alpha) * mask;"));
    }
}
