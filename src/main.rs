#[derive(Debug, Clone, Copy, PartialEq)]

struct Body {
    mass: f64,
    position: (f64, f64),
    velocity: (f64, f64),
}
impl Body {
    fn new(position: (f64, f64)) -> Self {
        Body {
            mass: 1.0,
            velocity: (0.0, 0.0),
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
    y:f64,
}

struct Step {
    time: f64,
    step_id: u32,
    bodies: [Body; 3],
}

const STEPS: usize = 1_000_000;
const TIME_STEP: f64 = 0.1;
const GRAVITATION_CONSTANT: f64 = 6.67430e-11; // m^3 kg^-1 s^-2

fn main() {
    let mut first = Body::new((0.3089693008, 0.4236727692));
    let mut second = Body::new((-0.5, 0.0));
    let mut third = Body::new((0.5, 0.0));

    let steps: Vec<Step> = Vec::<Step>::with_capacity(STEPS);

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
                    let force = GRAVITATION_CONSTANT * (new_step.bodies[i].mass * new_step.bodies[j].mass) / (r * r);
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

        if step_id % 1000 == 0 {
            println!(" 1st: ({:.05}, {:.05})", first.position.0, first.position.1);
            // print!("2nd: ({:.05}, {:.05})", second.position.0, second.position.1);
            // print!("3rd: ({:.05}, {:.05})", third.position.0, third.position.1);
        }
        // for i in 0..3 {
        //     new_step.bodies[i].position.0 += new_step.bodies[i].velocity.0 * TIME_STEP;
        //     new_step.bodies[i].position.1 += new_step.bodies[i].velocity.1 * TIME_STEP;            
        // }

        

    }
}
