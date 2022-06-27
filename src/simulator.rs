use std::time::Duration;

use hecs::{PreparedQuery, World};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, Sdl};

use crate::{
    components::{physics::*, render::RenderComponent},
    systems::*,
};

pub struct Simulator<'a> {
    fps: u32,
    world: World,
    ctx: &'a Sdl,
    canvas: Canvas<Window>,
    width: u32,
    height: u32
}

impl<'a> Simulator<'a> {
    pub fn new(width: u32, height: u32, fps: u32, ctx: &'a Sdl) -> Self {
        let video = ctx.video().unwrap();
        let window = video
            .window("Test", width, height)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        Simulator {
            fps,
            world: World::new(),
            ctx,
            canvas,
            width,
            height
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let mut event_pump = self.ctx.event_pump().unwrap();
        let entity = self.world.spawn((
            PositionComponent::new(200.0, 0.0),
            VelocityComponent::new(0.0, 0.0),
            AccelerationComponent::new(0.0, 200.0),
            MassComponent::new(1.0),
            RenderComponent::new(Color::RGB(255, 255, 255)),
        ));

        let entity2 = self.world.spawn((
            PositionComponent::new(200.0, 0.0),
            VelocityComponent::new(0.0, 0.0),
            AccelerationComponent::new(0.0, 500.0),
            MassComponent::new(1.0),
            RenderComponent::new(Color::RGB(20, 100, 120)),
        ));

        let mut frame = 0;
        'running: loop {
            frame += 1;
            // println!("FRAME: {}", frame);
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        self.world.spawn((
                            PositionComponent::new(200.0, 0.0),
                            VelocityComponent::new(200.0, 0.0),
                            AccelerationComponent::new(0.0, 500.0),
                            MassComponent::new(1.0),
                            RenderComponent::new(Color::RGB(255, 255, 255)),
                            SizeComponent::new(50, 50),
                            CollisionComponent::new(CollisionType::Elastic)
                        ));
                        self.world.spawn((
                            PositionComponent::new(400.0, 0.0),
                            VelocityComponent::new(-500.0, 0.0),
                            AccelerationComponent::new(0.0, 500.0),
                            MassComponent::new(1.0),
                            RenderComponent::new(Color::RGB(255, 255, 255)),
                            SizeComponent::new(50, 50),
                            CollisionComponent::new(CollisionType::Elastic)
                        ));
                        // hollow_box.add_force_angle(150.0, 45.0);
                    }
                    _ => {}
                }
            }
            let dt = DeltaTime(Duration::new(0, 1_000_000_000u32 / self.fps));
            // collision_system(
            //     &mut self.world,
            //     &mut PreparedQuery::<(
            //         &PositionComponent,
            //         &SizeComponent,
            //         &MassComponent,
            //         &CollisionComponent,
            //     )>::default(),
            //     self.width,
            //     self.height
            // );
            // Physics System and others
            physics_system(
                &mut self.world,
                &mut PreparedQuery::<(
                    &mut PositionComponent,
                    &mut VelocityComponent,
                    &mut AccelerationComponent,
                    &MassComponent,
                )>::default(),
                &dt,
            );

            cleanup_system(
                &mut self.world,
                &mut PreparedQuery::<(
                    &PositionComponent
                )>::default(),
                self.width,
                self.height
            );
            // Render System comes last
            render_system(
                &mut self.world,
                &mut PreparedQuery::<(&RenderComponent, &PositionComponent)>::default(),
                &mut self.canvas,
            )?;
            self.canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.fps));
        }

        Ok(())
    }

    fn spawn_simulation_entities(&mut self) {

    }
}
