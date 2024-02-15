use std::time::Duration;

use nannou::prelude::*;
use rand::Rng;

const HEIGHT: u32 = 720;
const WIDTH: u32 = 720;
const NUM_HEARTS: u32 = 50;
const SPEED: f32 = 0.25;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

enum GameState {
    Started,
    Won,
    Lost,
}

struct Model {
    mouse_position: Point2,
    hearts: Vec<Point2>,
    font_size: u32,
    last_updated: f32,
    state: GameState,
}

fn model(app: &App) -> Model {
    let mut rng = rand::thread_rng();
    app.new_window()
        .size(WIDTH, HEIGHT)
        .mouse_moved(mouse_moved)
        .build()
        .unwrap();

    let hearts = (0..NUM_HEARTS)
        .map(|_| {
            let x = rng.gen_range(-0.5 * (WIDTH as f32), 0.5 * WIDTH as f32);
            let y = rng.gen_range(-0.5 * (HEIGHT as f32), 0.5 * HEIGHT as f32);
            Point2::new(x, y)
        })
        .collect();
    Model {
        mouse_position: Point2::new(0.0, 0.0),
        hearts,
        font_size: 20,
        last_updated: 0.0,
        state: GameState::Started,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let current_num_hearts = model.hearts.len();
    if current_num_hearts > 0 && app.duration.since_start.as_secs_f32() > model.last_updated + SPEED
    {
        let current_num_hearts = model.hearts.len();
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0, current_num_hearts);
        let hearts = [
            &model.hearts[0..random_index],
            &model.hearts[random_index + 1..current_num_hearts],
        ]
        .concat();
        if hearts.is_empty() {
            model.state = GameState::Lost
        }
        model.hearts = hearts;
        model.last_updated = app.duration.since_start.as_secs_f32();
    }
}

fn mouse_moved(_app: &App, model: &mut Model, pos: Point2) {
    model.mouse_position = pos;

    let mut hearts: Vec<Point2> = Vec::new();
    for heart in model.hearts.clone() {
        if pos.distance(heart) < (model.font_size as f32) && model.font_size < 100 {
            model.font_size = model.font_size + 2;
        } else {
            hearts.push(heart);
        }
    }
    if hearts.len() == 0 {
        model.state = GameState::Won
    } else {
        model.hearts = hearts
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    match model.state {
        GameState::Started => {
            frame.clear(PINK);

            draw.text("heart")
                .font_size(model.font_size)
                .x(model.mouse_position.x)
                .y(model.mouse_position.y);

            for heart in &model.hearts {
                draw.text("heart").font_size(20).x(heart.x).y(heart.y);
            }
        }
        GameState::Won => {
            frame.clear(PURPLE);

            draw.text("a world of joy!").font_size(model.font_size);
        }
        GameState::Lost => {
            frame.clear(GRAY);

            draw.text("no more")
                .font_size(model.font_size)
                .x(model.mouse_position.x)
                .y(model.mouse_position.y);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
