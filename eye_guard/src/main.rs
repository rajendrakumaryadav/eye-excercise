use std::time::{Duration, Instant};
use notify_rust::Notification;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};

fn draw_circle(frame: &mut [u8], width: u32, height: u32, cx: i32, cy: i32, r: i32, color: [u8; 4]) {
    let r_sq = r * r;
    for y in (cy - r)..=(cy + r) {
        for x in (cx - r)..=(cx + r) {
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                let dx = x - cx;
                let dy = y - cy;
                if dx * dx + dy * dy <= r_sq {
                    let idx = ((y as u32 * width + x as u32) * 4) as usize;
                    frame[idx..idx + 4].copy_from_slice(&color);
                }
            }
        }
    }
}

fn run_overlay(duration: Duration) -> Result<(), pixels::Error> {
    let event_loop = EventLoop::new();
    let monitor = event_loop
        .primary_monitor()
        .or_else(|| event_loop.available_monitors().next());
    let size = monitor
        .as_ref()
        .map(|m| m.size())
        .unwrap_or(PhysicalSize::new(800, 600));

    let window = WindowBuilder::new()
        .with_title("Eye Guard")
        .with_fullscreen(Some(Fullscreen::Borderless(monitor)))
        .with_decorations(false)
        .with_always_on_top(true)
        .build(&event_loop)
        .expect("Failed to create window");

    let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
    let mut pixels = Pixels::new(size.width, size.height, surface_texture)?;

    let start = Instant::now();
    let mut t: f32 = 0.0;

    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {
                if start.elapsed() >= duration {
                    *control_flow = ControlFlow::Exit;
                } else {
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                let frame = pixels.frame_mut();
                for pix in frame.chunks_exact_mut(4) {
                    pix.copy_from_slice(&[0, 0, 0, 255]);
                }
                let width = size.width as f32;
                let height = size.height as f32;
                // simple figure-eight style motion
                t += 0.05;
                let x = (width / 2.0 + (width / 4.0) * t.cos()) as i32;
                let y = (height / 2.0 + (height / 4.0) * (2.0 * t).sin()) as i32;
                draw_circle(frame, size.width, size.height, x, y, 30, [0, 255, 0, 255]);
                pixels.render().unwrap();
            }
            _ => {}
        }
    });

    Ok(())
}

#[tokio::main]
async fn main() {
    loop {
        tokio::time::sleep(Duration::from_secs(30 * 60)).await;
        let _ = Notification::new()
            .summary("Eye Guard")
            .body("Time to rest your eyes!")
            .show();
        let _ = run_overlay(Duration::from_secs(25));
    }
}
