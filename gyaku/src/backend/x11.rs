use crate::event_loop::EventLoopData;
use crate::state::GyakuState;
use color_eyre::Result;
use slog::Logger;
use smithay::{
    backend::{
        renderer::{
            damage::DamageTrackedRenderer, element::surface::WaylandSurfaceRenderElement,
            gles2::Gles2Renderer, glow::{GlowFrame, GlowRenderer}, Bind,
        }, x11::{X11Backend, WindowBuilder, Window, X11Surface}, egl::{EGLDisplay, EGLContext},
    },
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::{calloop::timer::TimeoutAction, gbm},
    utils::{Rectangle, Transform, DeviceFd, Size, Logical},
};
use std::{time::Duration, collections::HashSet};

use super::Backend;

pub struct X11NestedBackend {
    log: Logger,
    output: Output,
    renderer: DamageTrackedRenderer,
    backend: X11Backend,
    full_redraw: u8,
    window: Window,
    glow_renderer: GlowRenderer,
    surface: X11Surface,
}

impl X11NestedBackend {
    pub fn new(data: &mut EventLoopData, log: Logger) -> Result<Self> {
        let display = &mut data.display;

        let backend = X11Backend::new(log.clone())?;

        let x_handle = backend.handle();

        let window = WindowBuilder::new()
            .title("Gyaku")
            .build(&x_handle)?;

        let (_drm_node, fd) = x_handle.drm_node()?;
        let device = gbm::Device::new(DeviceFd::from(fd))?;

        let egl = EGLDisplay::new(device.clone(), log.clone()).expect("Failed to create EGLDisplay");
        let context = EGLContext::new(&egl, log.clone()).expect("Failed to create EGLContext");
        let modifiers = context.dmabuf_render_formats().iter().map(|format| format.modifier).collect::<HashSet<_>>();
        
        let surface = x_handle.create_surface(&window, device, modifiers.into_iter())?;

        let glow_renderer = unsafe { GlowRenderer::new(context, log.clone())? };

        // TODO: Handle DMA buffer

        let size = {
            let s = window.size();
            (s.w as i32, s.h as i32).into()
        };

        let mode = Mode {
            size,
            refresh: 60_000,
        };

        let output = Output::new::<_>(
            "x11".to_string(),
            PhysicalProperties {
                size: (0, 0).into(),
                subpixel: Subpixel::Unknown,
                make: "Smithay".into(),
                model: "x11".into(),
            },
            log.clone(),
        );

        let _global = output.create_global::<GyakuState>(&display.handle());
        output.change_current_state(
            Some(mode),
            None,
            None,
            Some((0, 0).into()),
        );
        output.set_preferred(mode);

        data.state.space.map_output(&output, (0, 0));

        let damage_tracked_renderer = DamageTrackedRenderer::from_output(&output);

        Ok(Self {
            log,
            backend,
            output,
            renderer: damage_tracked_renderer,
            full_redraw: 0u8,
            window,
            glow_renderer,
            surface
        })
    }
}

impl Backend for X11NestedBackend {
    fn dispatch(&mut self, data: &mut EventLoopData) -> Result<TimeoutAction> {
        let display = &mut data.display;
        let state = &mut data.state;

        self.full_redraw = self.full_redraw.saturating_sub(1);

        let size: Size<i32, Logical> = {
            let s = self.window.size();
            (s.w as i32, s.h as i32).into()
        };
        let damage = Rectangle::from_loc_and_size((0, 0), size);

        let (buffer, _age) = self.surface.buffer()?;
        self.glow_renderer.bind(buffer)?;
        smithay::desktop::space::render_output::<
            _,
            WaylandSurfaceRenderElement<GlowRenderer>,
            _,
            _,
            _,
        >(
            &self.output,
            &mut self.glow_renderer,
            0,
            [&state.space],
            &[],
            &mut self.renderer,
            [0.1, 0.1, 0.1, 1.0],
            self.log.clone(),
        )?;
        self.surface.submit()?;

        state.space.elements().for_each(|window| {
            window.send_frame(
                &self.output,
                data.start_time.elapsed(),
                Some(Duration::ZERO),
                |_, _| Some(self.output.clone()),
            )
        });

        state.space.refresh();
        display.flush_clients()?;

        Ok(TimeoutAction::ToDuration(Duration::from_millis(16)))

    }
}
