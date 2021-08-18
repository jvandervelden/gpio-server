mod pwm_handler;
mod soft_pwm_handler;
mod switch_handler;
mod button_handler;

pub use switch_handler::SwitchHandler;
pub use soft_pwm_handler::SoftPwmHandler;
pub use pwm_handler::PwmHandler;
pub use button_handler::ButtonHandler;

pub trait Handler {
    fn get_pin(&self) -> u16;
    fn get_value(&self) -> f32;
    fn get_type(&self) -> &str;
    fn set_value(&mut self, value: f32);
    fn start(&mut self);
    fn stop(&mut self);
}
