use piston_window::*;
use rand::*;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;

struct Bubble {
    speed: f64,
    x: f64,
    y: f64,
    r: f64,
}

impl Bubble {
    pub fn new(num: Option<f64>) -> Bubble {
        let r = (random::<f64>() * (WIDTH / 8.0)) + 5.0;
        let mut bubble: Bubble = Bubble {
            speed: (random::<f64>() * 90.0) + 10.0,
            x: random::<f64>() * WIDTH,
            y: random::<f64>() * (HEIGHT + r),
            r,
        };
        if let Some(y) = num {
            bubble.speed = 0.0;
            bubble.y = y;
        }
        bubble
    }
}

fn get_bubbles() -> Vec<Bubble> {
    let mut bubbles = Vec::new();

    let n = (random::<u64>() % 15) + 10;

    for _ in 0..n {
        bubbles.push(Bubble::new(Some(HEIGHT)));
        bubbles.push(Bubble::new(Some(0.0)));
        bubbles.push(Bubble::new(None));
    }

    bubbles
}

fn main() -> () {
    let bubble_color = [ 50.0, 10.0/255.0, 50.0, 1.0 ];
    let background_color = [ 200.0/255.0, 10.0/255.0, 150.0/255.0, 1.0 ];

    let mut bubbles: Vec<Bubble> = get_bubbles();

    let mut window: PistonWindow = WindowSettings::new("Lava Lamp", [ WIDTH, HEIGHT ])
        .exit_on_esc(true).build().unwrap();

    let mut events = window.events;

    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            let bubbles_ref = &bubbles;
            window.draw_2d(&e, |c, g, _| {
                clear(background_color, g);
                for b in bubbles_ref {
                    ellipse(bubble_color, [b.x - b.r, b.y - b.r, b.r * 2.0, b.r * 2.0 ], c.transform, g);
                }
            });
        }
        if let Some(u) = e.update_args() {
            let bubbles_ref = &mut bubbles;
            for b in bubbles_ref {
                b.y -= b.speed * u.dt;
                if b.y + b.r <= 0.0 {
                    b.y = HEIGHT + b.r;
                }
            }
        }
    }
}
