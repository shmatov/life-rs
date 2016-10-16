#![feature(conservative_impl_trait)]
#![feature(inclusive_range_syntax)]


use std::thread;
use std::time::Duration;


mod iterator_ext;
mod core;
mod graphics;


struct Game<E: ::core::Engine, G: ::graphics::Graphics> {
    graphics: G,
    engine: E,
    timeout: Duration,
}



impl<E: ::core::Engine, G: ::graphics::Graphics> Game<E, G> {
    fn new(engine: E, graphics: G, timeout: Duration) -> Self {
        Game { engine: engine, graphics: graphics, timeout: timeout}
    }

    fn run(&mut self) {
        loop {
            self.graphics.visualize_state(self.engine.get_state());
            thread::sleep(self.timeout);
            self.engine = self.engine.make_step();
        }
    }
}


fn glider() -> Vec<::core::Cell> {
    vec![
                (0, 1),
                        (1, 2),
        (2, 0), (2, 1), (2, 2),
    ]
}


fn main() {
    let engine = ::core::Life::new(glider());
    let interface = ::graphics::CUI::new((0, 0), (40, 40));

    let mut game = Game::new(engine, interface, Duration::from_millis(150));
    game.run();
}
