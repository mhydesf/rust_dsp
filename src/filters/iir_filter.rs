pub trait Number: PartialOrd + num_traits::Signed + Copy {}
impl<T: PartialOrd + num_traits::Signed + Copy> Number for T {}

pub struct IIRFilter<T: Number> {
    alpha: T,
    alpha_i: T,
    intial_value: Option<T>,
    buffer: T,
    initialized: bool,
}

impl<T> IIRFilter<T>
where
    T: Number,
{
    // Generate+Initialize New IIR Filter
    pub fn new() -> Self {
        Self {
            alpha: T::zero(),
            alpha_i: T::zero(),
            intial_value: None,
            buffer: T::zero(),
            initialized: false,
        }
    }

    // Assign alpha
    pub fn alpha(&mut self, alpha: T) {
        assert!(alpha <= T::one());
        self.alpha = alpha;
        self.alpha_i = T::one() - alpha;
    }

    // Reset IIR Filter
    pub fn reset(&mut self) {
        self.initialized = false;
        if self.intial_value.is_some() {
            self.initialize_buffer(self.intial_value.unwrap());
        }
    }

    // Update IIR Filter
    pub fn update(&mut self, value: T) -> T {
        if !self.initialized {
            self.initialize_buffer(value);
        }
        self.buffer = self.alpha * value + self.alpha_i * self.buffer;
        self.buffer
    }

    // Initialized buffer value
    fn initialize_buffer(&mut self, value: T) {
        self.buffer = value;
        self.initialized = true;
    }
}
