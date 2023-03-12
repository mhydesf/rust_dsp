use std::thread::sleep;
use std::time::Duration;

use crate::controllers::pid_controller::PIDController;
use crate::filters::iir_filter::IIRFilter;
use crate::types::types::Limits;

pub mod controllers;
pub mod filters;
pub mod types;

fn test_pid() {
    let mut controller: PIDController<f32> = PIDController::new();
    let mut filter: IIRFilter<f32> = IIRFilter::new();
    filter.alpha(0.5);
    controller.setpoint(100.0);
    controller.output_limits(Limits {
        lower_limit: (0.0),
        upper_limit: (100.0),
    });
    controller.p(
        1.0,
        Limits {
            lower_limit: 0.0,
            upper_limit: 1.0,
        },
    );
    controller.i(
        1.0,
        Limits {
            lower_limit: 0.0,
            upper_limit: 1.0,
        },
    );
    controller.d(
        1.0,
        Limits {
            lower_limit: 0.0,
            upper_limit: 1.0,
        },
    );
    controller.filter(filter);

    for x in 1..150 {
        let s = controller.update(x as f32);
        println!("{}", s);
        sleep(Duration::from_millis(10));
    }

    controller.reset();
}

fn test_filter() {
    let mut filter: IIRFilter<f32> = IIRFilter::new();
    filter.alpha(0.5);
    for x in 1..5 {
        let s = filter.update(x as f32);
        println!("{}", s);
        sleep(Duration::from_millis(10));
    }
}

fn main() {
    test_pid();
    test_filter();
}
