mod soft_pwm_handler;
mod switch_handler;

pub use switch_handler::SwitchHandler;
pub use soft_pwm_handler::SoftPwmHandler;

use async_trait::async_trait;

#[async_trait]
pub trait Handler {
    fn set_value(&mut self, value: f32);
    async fn start(&mut self);
    fn stop(&mut self);
}
