mod application;
mod ui;
mod modules;
mod renderer;
mod simulator;
mod config;
mod fatal;
mod project;
mod selection;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

use adw::prelude::ApplicationExtManual;
use application::Application;

fn main() {
    env_logger::init();
    info!("Starting up LogicRs...");    
    
    let application = Application::new();
    std::process::exit(application.run());
}
