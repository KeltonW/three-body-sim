#![allow(unused)]

use plotters::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]

struct Body {
    mass: f64,
    position: (f64, f64),
    velocity: (f64, f64),
}
impl Body {
    fn new(position: (f64, f64), mass: f64) -> Self {
        Body {
            mass,
            velocity: (0.0, 0.0), // Initial velocity
            position,
        }
    }
}

struct Position {
    x: f64,
    y: f64,
}

struct Velocity {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
struct Step {
    time: f64,
    step_id: u32,
    bodies: [Body; 3],
}

const STEPS: usize = 1000000;
const TIME_STEP: f64 = 20.0;
const GRAVITATION_CONSTANT: f64 = 6.67430e-11; // m^3 kg^-1 s^-2

const FPS: u32 = 180;
const ANIMATION_LENGTH: u32 = 60; // s

fn animate_steps(steps: &[Step]) {
    let area = BitMapBackend::gif("three_body.gif", (250, 250), 1000 / FPS)
        .unwrap()
        .into_drawing_area();

    let mut ctx = ChartBuilder::on(&area)
        .build_cartesian_2d(-100..100, -100..100)
        .unwrap();

    for step_id in steps {
        area.fill(&WHITE).unwrap();
        ctx.configure_mesh().draw().unwrap();

        for n in 0..3 {
            let colour = match n {
                0 => BLUE,
                1 => RED,
                2 => GREEN,
                _ => BLACK,
            };
            // step_id.clone();
            ctx.draw_series([step_id].iter().map(|step| {
                Circle::new(
                    (
                        (step.bodies[n].position.0 * 100.0).round() as i32,
                        (step.bodies[n].position.1 * 100.0).round() as i32,
                    ),
                    2,
                    colour.filled(),
                )
            }))
            .unwrap();
        }
        area.draw(&Text::new(
            format!("T : {}", step_id.time.round() as u32), // step.time
            (5, 5),
            ("Inter", 12),
        ))
        .unwrap();
        area.present().unwrap();
    }
}

fn main() {
    let mut first = Body::new((0.3089693008, 0.4236727692), 1.0);
    let mut second = Body::new((-0.5, 0.0), 10.0);
    let mut third = Body::new((0.5, 0.0), 1.0);

    let mut steps: Vec<Step> = Vec::<Step>::with_capacity(STEPS);

    for step_id in 0..STEPS {
        let mut new_step = Step {
            time: (step_id as f64) * TIME_STEP,
            step_id: step_id as u32,
            bodies: [first.clone(), second.clone(), third.clone()],
        };
        for i in 0..3 {
            for j in 0..3 {
                if i != j {
                    let dx = new_step.bodies[j].position.0 - new_step.bodies[i].position.0;
                    let dy = new_step.bodies[j].position.1 - new_step.bodies[i].position.1;

                    let r = (dx * dx + dy * dy).sqrt();
                    let force = GRAVITATION_CONSTANT
                        * (new_step.bodies[i].mass * new_step.bodies[j].mass)
                        / (r * r);
                    let angle = dy.atan2(dx);
                    let fx = force * angle.cos();
                    let fy = force * angle.sin();
                    new_step.bodies[i].velocity.0 += fx / new_step.bodies[i].mass * TIME_STEP;
                    new_step.bodies[i].velocity.1 += fy / new_step.bodies[i].mass * TIME_STEP;
                }
            }
        }

        for body in new_step.bodies.iter_mut() {
            body.position.0 += body.velocity.0 * TIME_STEP;
            body.position.1 += body.velocity.1 * TIME_STEP;
        }

        first = new_step.bodies[0].clone();
        second = new_step.bodies[1].clone();
        third = new_step.bodies[2].clone();

        // if step_id % 1000 == 0 {
        //     println!(" 1st: ({:.05}, {:.05})", first.position.0, first.position.1);
            // print!("2nd: ({:.05}, {:.05})", second.position.0, second.position.1);
            // print!("3rd: ({:.05}, {:.05})", third.position.0, third.position.1);
            steps.push(new_step.clone());
        }
        // for i in 0..3 {
        //     new_step.bodies[i].position.0 += new_step.bodies[i].velocity.0 * TIME_STEP;
        //     new_step.bodies[i].position.1 += new_step.bodies[i].velocity.1 * TIME_STEP;
        // }
        animate_steps(&steps);
    }
