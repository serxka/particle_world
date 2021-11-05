use sdl2::{
	event::{Event, WindowEvent},
	keyboard::Keycode,
	render::{Canvas, RenderTarget},
};
use std::time::{Duration, Instant};

use sim::{World, WorldParameters};

const TARGET_FPS: u128 = 60;

const PARAMS_DEF: WorldParameters = WorldParameters {
	bounds: (100.0, 100.0),
	attrac_mean: 0.02,
	attrac_dev: 0.04,
	attrac_range: 1.0..50.0,
	radius_range: 0.7..=1.3,
	friction: 0.05,
	seed: String::new(),
};

fn main() -> Result<(), String> {
	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;

	let window = video_subsystem
		.window("particle world", 1280, 1280)
		.position_centered()
		.resizable()
		.opengl()
		.build()
		.map_err(|e| e.to_string())?;

	let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
	let mut size = (1280, 1280);

	let mut para = PARAMS_DEF.clone();
	para.seed = "".into();
	let mut world = World::new(150, 7, para);
	let mut paused = false;
	let mut steps: usize = 10;

	let mut event = sdl_context.event_pump()?;
	'running: loop {
		let time = Instant::now();
		let next_time = time.elapsed().as_nanos() + (1_000_000_000u128 / TARGET_FPS);
		for event in event.poll_iter() {
			match event {
				Event::Quit { .. } => break 'running,
				Event::KeyDown {
					keycode: Some(key), ..
				} => match key {
					Keycode::Q => break 'running,
					Keycode::R => world.reseed(PARAMS_DEF),
					Keycode::Space => paused = !paused,
					Keycode::Left => steps = steps.saturating_sub(1).max(1),
					Keycode::Right => steps = (steps + 1).min(100),
					Keycode::Up => world.add_particle(),
					Keycode::Down => world.del_particle(),
					_ => {}
				},
				Event::Window {
					win_event: WindowEvent::Resized(w, h),
					..
				} => size = (w, h),
				_ => {}
			}
		}
		canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
		canvas.clear();

		let window_size = size.0.min(size.1);
		let scale = window_size as f32 / 100.0;
		for part in &world.parts {
			let col = world.kinds.colour(part.kind);
			canvas.set_draw_color(sdl2::pixels::Color::RGB(col.0, col.1, col.2));

			let pos = (
				(part.pos.x * scale) as i32,
				(part.pos.y * scale) as i32,
			);
			draw_circle(&mut canvas, pos, (part.rad * scale) as i32);
		}

		canvas.present();
		if !paused {
			for _ in 0..steps {
				world.step();
			}
		}
		let elapsed = time.elapsed().as_nanos();
		if next_time > elapsed {
			std::thread::sleep(Duration::new(0, (next_time - elapsed) as u32));
		}
	}
	Ok(())
}

fn draw_circle<T: RenderTarget>(canvas: &mut Canvas<T>, centre: (i32, i32), radius: i32) {
	let diameter = radius * 2;
	for x in 0..diameter {
		for y in 0..diameter {
			let dx = radius - x;
			let dy = radius - y;
			if (dx * dx + dy * dy) <= (radius * radius) {
				canvas.draw_point((centre.0 + dx, centre.1 + dy)).unwrap();
			}
		}
	}
}
