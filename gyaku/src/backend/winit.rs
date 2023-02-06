use crate::event_loop::EventLoopData;
use crate::state::GyakuState;
use color_eyre::Result;
use slog::Logger;
use smithay::{
    backend::{
        renderer::{
            damage::DamageTrackedRenderer, element::surface::WaylandSurfaceRenderElement,
            glow::GlowRenderer,
        },
        winit::{self, WinitError, WinitEvent, WinitEventLoop, WinitGraphicsBackend},
    },
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::calloop::timer::TimeoutAction,
    utils::{Rectangle, Transform},
};
use std::time::Duration;

use super::Backend;

pub struct WinitBackend {
    log: Logger,
    output: Output,
    renderer: DamageTrackedRenderer,
    winit_backend: WinitGraphicsBackend<GlowRenderer>,
    winit_event_loop: WinitEventLoop,
    full_redraw: u8,
}

impl WinitBackend {
    pub fn new(data: &mut EventLoopData, log: Logger) -> Result<Self> {
        let display = &mut data.display;
        let state = &mut data.state;

        let (winit_backend, winit_event_loop) =
            winit::init::<GlowRenderer, _>(log.clone()).unwrap();

        let mode = Mode {
            size: winit_backend.window_size().physical_size,
            refresh: 60_000,
        };

        let output = Output::new::<_>(
            "winit".to_string(),
            PhysicalProperties {
                size: (0, 0).into(),
                subpixel: Subpixel::Unknown,
                make: "Smithay".into(),
                model: "Winit".into(),
            },
            log.clone(),
        );

        let _global = output.create_global::<GyakuState>(&display.handle());
        output.change_current_state(
            Some(mode),
            Some(Transform::Flipped180),
            None,
            Some((0, 0).into()),
        );
        output.set_preferred(mode);

        state.space.map_output(&output, (0, 0));

        let damage_tracked_renderer = DamageTrackedRenderer::from_output(&output);

        Ok(WinitBackend {
            log: log.clone(),
            output,
            renderer: damage_tracked_renderer,
            winit_backend,
            winit_event_loop,
            full_redraw: 0u8,
        })
    }
}

impl Backend for WinitBackend {
    fn dispatch(&mut self, data: &mut EventLoopData) -> Result<TimeoutAction> {
        let display = &mut data.display;
        let state = &mut data.state;

        let res = self
            .winit_event_loop
            .dispatch_new_events(|event| match event {
                WinitEvent::Resized { size, .. } => {
                    self.output.change_current_state(
                        Some(Mode {
                            size,
                            refresh: 60_000,
                        }),
                        None,
                        None,
                        None,
                    );
                }
                WinitEvent::Input(event) => state.dispatch_input_event(event),
                _ => (),
            });

        if let Err(WinitError::WindowClosed) = res {
            // Stop the loop
            //state.loop_signal.stop();

            return Ok(TimeoutAction::Drop);
        } else {
            res?;
        }

        self.full_redraw = self.full_redraw.saturating_sub(1);

        let size = self.winit_backend.window_size().physical_size;
        let damage = Rectangle::from_loc_and_size((0, 0), size);

        self.winit_backend.bind()?;
        smithay::desktop::space::render_output::<
            _,
            WaylandSurfaceRenderElement<GlowRenderer>,
            _,
            _,
            _,
        >(
            &self.output,
            self.winit_backend.renderer(),
            0,
            [&state.space],
            &[],
            &mut self.renderer,
            [0.1, 0.1, 0.1, 1.0],
            self.log.clone(),
        )?;
        self.winit_backend.submit(Some(&[damage]))?;

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
