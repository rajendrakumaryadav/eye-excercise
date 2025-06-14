use std::error::Error;
use std::time::{Duration, Instant};

use notify_rust::Notification;
use pixels::{Pixels, SurfaceTexture};
use tokio::time::sleep;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Fullscreen, WindowBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        sleep(Duration::from_secs(30 * 60)).await;

        // Notify the user
        let _ = Notification::new()
            .summary("Eye Guard")
            .body("Time to rest your eyes!")
            .show();

        // Run the overlay animation
        run_overlay(Duration::from_secs(25)).await;
    }
}

async fn run_overlay(duration: Duration) {
    // Spawn blocking since winit's event loop blocks the thread
    let _ = tokio::task::spawn_blocking(move || {
        start_overlay(duration)
    })
    .await;
}

fn start_overlay(duration: Duration) {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Eye Break")
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_decorations(false)
        .with_always_on_top(true)
        .build(&event_loop)
        .expect("failed to create window");

    let size = window.inner_size();
    let mut pixels = {
        let surface = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(size.width, size.height, surface).expect("pixels init")
    };

    let start = Instant::now();
    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => {
                pixels.resize_surface(new_size.width, new_size.height);
            }
            Event::RedrawRequested(_) => {
                draw_frame(pixels.get_frame(), pixels.width(), pixels.height(), start.elapsed());
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
                if start.elapsed() > duration {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}

fn draw_frame(frame: &mut [u8], width: u32, height: u32, elapsed: Duration) {
    for chunk in frame.chunks_exact_mut(4) {
        chunk[0] = 0;
        chunk[1] = 0;
        chunk[2] = 0;
        chunk[3] = 255;
    }

    let t = elapsed.as_secs_f32();
    let amp = width.min(height) as f32 / 4.0;
    let x = width as f32 / 2.0 + amp * (t).sin();
    let y = height as f32 / 2.0 + amp * (t * 0.5).sin() * (t * 0.5).cos();

    let xi = x as i32.clamp(0, width as i32 - 1);
    let yi = y as i32.clamp(0, height as i32 - 1);
    let idx = ((yi as u32 * width + xi as u32) * 4) as usize;
    if idx + 4 <= frame.len() {
        frame[idx] = 255;
        frame[idx + 1] = 255;
        frame[idx + 2] = 255;
        frame[idx + 3] = 255;
    }
}

