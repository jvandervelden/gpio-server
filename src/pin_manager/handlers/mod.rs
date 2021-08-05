mod pwm_handler;
mod soft_pwm_handler;
mod switch_handler;

pub use switch_handler::SwitchHandler;
pub use soft_pwm_handler::SoftPwmHandler;
pub use pwm_handler::PwmHandler;

pub trait Handler {
    fn set_value(&mut self, value: f32);
    fn start(&mut self);
    fn stop(&mut self);
}
