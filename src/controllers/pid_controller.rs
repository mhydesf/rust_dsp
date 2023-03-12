use crate::types::types::Limits;
use num::{cast, Num, NumCast};
use std::time::SystemTime;

pub trait Number: PartialOrd + num_traits::Signed + Copy {}
impl<T: PartialOrd + num_traits::Signed + Copy> Number for T {}

pub struct PIDController<T: Number> {
    pub setpoint: T,
    pub kp: T,
    pub ki: T,
    pub kd: T,
    pub p_limits: Limits<T>,
    pub i_limits: Limits<T>,
    pub d_limits: Limits<T>,
    pub output_limits: Limits<T>,
    band_limit_i: T,
    prev_time: SystemTime,
    prev_error: T,
    first_sample: bool,
    ei: T,
}

impl<T> PIDController<T>
where
    T: Number,
    T: Num + NumCast,
{
    // Create+Initialize new PIDController
    pub fn new() -> Self {
        Self {
            setpoint: T::zero(),
            output_limits: Limits {
                lower_limit: T::zero(),
                upper_limit: T::zero(),
            },
            kp: T::zero(),
            ki: T::zero(),
            kd: T::zero(),
            p_limits: Limits {
                lower_limit: (T::zero()),
                upper_limit: (T::zero()),
            },
            i_limits: Limits {
                lower_limit: (T::zero()),
                upper_limit: (T::zero()),
            },
            d_limits: Limits {
                lower_limit: (T::zero()),
                upper_limit: (T::zero()),
            },
            band_limit_i: T::zero(),
            prev_time: SystemTime::now(),
            prev_error: T::zero(),
            first_sample: true,
            ei: T::zero(),
        }
    }

    // Load Controller Specific Settings
    pub fn load_settings(
        &mut self,
        output_limits: Limits<T>,
        kp: T,
        ki: T,
        kd: T,
        p_limits: Limits<T>,
        i_limits: Limits<T>,
        d_limits: Limits<T>,
        band_limit_i: T,
    ) {
        self.setpoint = T::zero();
        self.prev_time = SystemTime::now();
        self.prev_error = T::zero();
        self.first_sample = true;
        self.ei = T::zero();

        self.output_limits = output_limits;
        self.kp = kp;
        self.ki = ki;
        self.kd = kd;
        self.p_limits = p_limits;
        self.i_limits = i_limits;
        self.d_limits = d_limits;
        self.band_limit_i = band_limit_i;
    }

    // Reset the controller to base state
    pub fn reset(&mut self) {
        self.setpoint = T::zero();
        self.output_limits = Limits {
            lower_limit: T::zero(),
            upper_limit: T::zero(),
        };
        self.kp = T::zero();
        self.ki = T::zero();
        self.kd = T::zero();
        self.p_limits = Limits {
            lower_limit: T::zero(),
            upper_limit: T::zero(),
        };
        self.i_limits = Limits {
            lower_limit: T::zero(),
            upper_limit: T::zero(),
        };
        self.d_limits = Limits {
            lower_limit: T::zero(),
            upper_limit: T::zero(),
        };
        self.band_limit_i = T::zero();
        self.prev_time = SystemTime::now();
        self.prev_error = T::zero();
        self.first_sample = true;
        self.ei = T::zero()
    }

    // Set Proportional Gain and Limits
    pub fn p(&mut self, gain: T, limits: Limits<T>) -> &mut Self {
        self.kp = gain;
        self.p_limits = limits;
        self
    }

    // Set Integral Gain and Limits
    pub fn i(&mut self, gain: T, limits: Limits<T>) -> &mut Self {
        self.ki = gain;
        self.i_limits = limits;
        self
    }

    // Set Derivative Gain and Limits
    pub fn d(&mut self, gain: T, limits: Limits<T>) -> &mut Self {
        self.kd = gain;
        self.d_limits = limits;
        self
    }

    // Set Controller Setpoint
    pub fn setpoint(&mut self, setpoint: T) -> &mut Self {
        self.setpoint = setpoint;
        self
    }

    // Set Controller Output Limits
    pub fn output_limits(&mut self, output_limits: Limits<T>) -> &mut Self {
        self.output_limits = output_limits;
        self
    }

    // Discrete Controller Update
    pub fn update(&mut self, value: T) -> T {
        let now = SystemTime::now();
        let dt = now.duration_since(self.prev_time).expect("msg");

        let error = self.setpoint - value;
        let mut result: T;

        if self.first_sample {
            result = T::zero();
        } else {
            // P
            let ep = error * self.kp;
            // D
            let mut ed = (error - self.prev_error) / cast(dt.as_secs()).unwrap();
            ed = num_traits::clamp(
                ed * self.kd,
                self.d_limits.lower_limit,
                self.d_limits.upper_limit,
            );

            if error.abs() < self.band_limit_i {
                self.ei = num_traits::clamp(
                    self.ei + error * cast(dt.as_secs()).unwrap() * self.ki,
                    self.i_limits.lower_limit,
                    self.i_limits.upper_limit,
                );
            } else {
                self.ei = T::zero();
            }

            result = ep + ed + self.ei;
        }
        self.prev_time = now;
        self.prev_error = error;
        self.first_sample = false;

        result = num_traits::clamp(
            result,
            self.output_limits.lower_limit,
            self.output_limits.upper_limit,
        );

        result
    }
}
