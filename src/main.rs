extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;

fn get_eq(pos1: (f32, f32), pos2: (f32, f32)) -> (f32, f32) {
    let mut k = 0.0;
    let mut c = 0.0;

    if pos2.0 == pos1.0 {
        k = 0.0;
    } else {
        if pos2.1 == pos1.1 {
            k = 0.0;
        } else {
            k = (pos2.1 - pos1.1) / (pos2.0 - pos1.0);
        }

        c += pos1.1;
        c -= pos1.0 * k;
    }

    return (k, c);
}

fn draw_line(pos1: (f32, f32), pos2: (f32, f32), canvas: &mut WindowCanvas) {
    let line = get_eq(pos1, pos2);

    let k = line.0;
    let c = line.1;

    let offset = 50.0;

    if k == 0.0 {
        if pos2.0 == pos1.0 {
            let mut y = pos1.1;
            while y < (pos2.1) {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.fill_rect(Rect::new(
                    (pos1.0 + offset) as i32,
                    (y + offset) as i32,
                    3,
                    3,
                ));

                y = y + 1.0;
            }
            return;
        }
    }

    let mut x = pos1.0;
    while x < pos2.0 {
        let y = k * x + c + offset;

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new((x + offset) as i32, y as i32, 3, 3));

        x = x + 1.0;
    }
}


fn line_intersection(
    line1: (f32, f32),
    line2: (f32, f32),
    canvas: &mut WindowCanvas
    ) {

    //https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection

    let x = (line2.1 - line1.1)/(line1.0-line2.0) + 50.0;
    let y = line1.0*(line2.1 - line1.1)/(line1.0-line2.0) + line1.1 + 50.0;

    println!("{} {}", x, y);

    canvas.set_draw_color(Color::RGB(100, 100, 100));
    canvas.fill_rect(Rect::new(x as i32, y as i32, 7, 7));
}

pub fn get_points(ratio: f32, heigth: f32) {

    let mut points = vec![];
    //first line

    let mut i = 0;
    let mut j = 0;
    while i <= 3 {
        points.push((0.0, 0.0));
        points.push(((heigth * ratio), 0.0));
        points.push(((heigth * (1.0 - ratio), 0.0)));
    }
     
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 600, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let width = 550;
    let heigth = 550;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        draw_line((0.0, 0.0), (500.0, 0.0), &mut canvas);
        draw_line((0.0, 0.0), (0.0, 500.0), &mut canvas);
        draw_line((0.0, 500.0), (500.0, 500.0), &mut canvas);
        draw_line((500.0, 0.0), (500.0, 500.0), &mut canvas);

        draw_line((0.0, 100.0), (500.0, 400.0), &mut canvas);
        draw_line((100.0, 0.0), (500.0, 500.0), &mut canvas);

        let line1 = get_eq((0.0, 100.0), (500.0, 400.0));
        let line2 = get_eq((100.0, 0.0), (500.0, 500.0));

        line_intersection(line1, line2, &mut canvas);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
