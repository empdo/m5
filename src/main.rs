extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::collections::HashMap;
use std::time::Duration;
use std::vec;

fn distance(pos1: (f32, f32), pos2: (f32, f32)) -> f32 {
    return ((pos2.0 - pos1.0).powi(2) + (pos2.1 - pos1.1).powi(2)).sqrt();
}

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

        c += pos1.1 - pos1.0 * k;
    }

    return (k, c);
}

fn draw_line(pos1: (f32, f32), pos2: (f32, f32), canvas: &mut WindowCanvas) -> (f32, f32) {
    let mut p1 = pos1;
    let _p1 = pos1;
    let mut p2 = pos2;

    if (p1.0 > p2.0 || p1.0 < p1.1) {
        p1 = pos2;
        p2 = _p1;
    }

    let line = get_eq(p1, p2);

    let k = line.0;
    let c = line.1;

    let offset = 50.0;

    if k == 0.0 {
        if p2.0 == p1.0 {
            let mut y = p1.1;
            while y < (p2.1) {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.fill_rect(Rect::new((p1.0 + offset) as i32, (y + offset) as i32, 3, 3));

                y = y + 1.0;
            }
            return line;
        }
    }

    let mut x = p1.0;
    while x < p2.0 {
        let y = k * x + c + offset;

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new((x + offset) as i32, y as i32, 3, 3));

        x = x + 1.0;
    }

    return line;
}

fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}

fn line_intersection(
    line1: (f32, f32), //k, c
    line2: (f32, f32),
    canvas: &mut WindowCanvas,
) -> (f32, f32) {
    //https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection

    let x = (line2.1 - line1.1) / (line1.0 - line2.0) + 50.0;
    let y = line1.0 * (line2.1 - line1.1) / (line1.0 - line2.0) + line1.1 + 50.0;

//    canvas.set_draw_color(Color::RGB(250, 0, 0));
//    canvas.fill_rect(Rect::new((x - 1.5) as i32, (y - 1.5) as i32, 6, 6));

    return (round(x, 1), round(y, 1));
}

pub fn get_points(ratio: f32, heigth: f32) -> Vec<(f32, f32)> {
    let mut points = vec![];

    let p = ratio * heigth;
    let p2 = (1.0 - ratio) * heigth;
    points.push((0.0, 0.0));
    points.push((p, 0.0));
    points.push((heigth, 0.0));
    //points.push((heigth, p));
    points.push((heigth, heigth / 2.0));
    points.push((heigth, heigth));
    //points.push((p2, heigth));
    points.push((p2, heigth));
    points.push((0.0, heigth));
    points.push((0.0, heigth / 2.0));

    return points;
}

fn octagon_area(points: &Vec<(f32, f32)>) -> f32 {
    // Check that we have exactly 8 points
    assert_eq!(points.len(), 8);

    // Calculate the area of the octagon
    let area = 0.5 * points.iter().enumerate().fold(0.0, |acc, (i, p)| {
        let j = (i + 1) % 8;
        acc + (points[j].0 + p.0) * (points[j].1 - p.1)
    });

    area.abs()
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

    let mut frame = 0f32;
    let mut event_pump = sdl_context.event_pump().unwrap();
    println!("{0: <10} | {1: <10}", "Area", "Ratio");

    //    let mut a = vec![];

    'running: loop {
        let ratio = 0.4 + (frame * 0.1 % 20.0) / 100.0;

        let mut lines: Vec<(f32, f32)> = vec![];

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(0, 0, width, heigth));

        let points = get_points(ratio, (heigth - 50) as f32);
        for (index, point) in points.iter().enumerate() {
            if points.len() > index + 1 {
                draw_line(*point, points[index + 1], &mut canvas);
            }
        }
        draw_line(points[points.len() - 1], points[0], &mut canvas);

        let length = points.len() as i32;
        let line = draw_line(
            points[(2) as usize],
            points[(length - 1) as usize],
            &mut canvas,
        );
        lines.push(line);
        for (prev, next) in points.iter().zip(1..length) {
            let point = points[((next + 2) % length) as usize];
            if prev.0 < point.0 {
                let line = draw_line(point, *prev, &mut canvas);
                lines.push(line);
            } else {
                let line = draw_line(*prev, point, &mut canvas);
                lines.push(line);
            }
        }

        // while i < length {
        //     let point;

        //     if i % 2 == 0 {
        //         point = points[(i - 3).rem_euclid(length) as usize];
        //     } else {
        //         point = points[(i + 3).rem_euclid(length) as usize];
        //     }
        //     let line = draw_line(points[i as usize], point, &mut canvas);
        //     lines.push(line);

        //     i += 1;
        // }

        let mut intersections: Vec<(f32, f32)> = vec![];
        for line in lines.iter() {
            if line.0 != 0.0 {
                let mut _intersections: Vec<(f32, f32)> = vec![];
                for _line in lines.iter() {
                    if round(line.0, 1) != round(_line.0, 1) && line.0 != 0.0 {
                        let intersect = line_intersection(*line, *_line, &mut canvas);
                        if !intersections.contains(&intersect) {
                            _intersections.push(intersect);
                        }
                    }
                }

                _intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
                _intersections.drain(0..2);
                _intersections.drain((_intersections.len() - 2 ).._intersections.len());

                for intersection in &_intersections{
                    canvas.set_draw_color(Color::RGB(250, 0, 0));
                    canvas.fill_rect(Rect::new((intersection.0 - 1.5) as i32, (intersection.1 - 1.5) as i32, 6, 6));
                }

                intersections.append(&mut _intersections);
                
            }
        }

        let area = octagon_area(&intersections);
        println!("{0: <10} ", area / ((width - 50) * (heigth - 50)) as f32);
        //println!("{0: <10} | {1: <10}", area, width * heigth);

        
        //  intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
        //  if intersections.len() > 1 && frame as i32 % 20 == 0{
        //     // println!(
        //     //     "{0: <15} | {1: <15}",
        //     //     distance(intersections[0], intersections[1]).powi(2) / ((width - 50) * (heigth - 50)) as f32,
        //     //     round(ratio, 3)
        //     // );

        //      a.push((
        //          (width * heigth) as f32 / distance(intersections[0], intersections[1]).powi(2),
        //          ratio));
        //  }

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
        //
        frame += 1.0;

        if ratio > 0.99 {
            break 'running;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

