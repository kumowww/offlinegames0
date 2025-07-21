extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

const FENSTER_GROESSE: (u32, u32) = (400, 400);
const BLOCK_GROESSE: u32 = 20;

#[derive(Clone, PartialEq)]
enum Richtung {
    Oben,
    Unten,
    Links,
    Rechts,
}

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}

struct Schlange {
    koerper: Vec<Position>,
    richtung: Richtung,
}

impl Schlange {
    fn neue() -> Self {
        let start_x = (FENSTER_GROESSE.0 / BLOCK_GROESSE / 2) as i32;
        let start_y = (FENSTER_GROESSE.1 / BLOCK_GROESSE / 2) as i32;
        Self {
            koerper: vec![Position { x: start_x, y: start_y }],
            richtung: Richtung::Rechts,
        }
    }
    fn bewege(&mut self) {
        let mut neuer_kopf = self.koerper[0].clone();
        match self.richtung {
            Richtung::Oben => neuer_kopf.y -= 1,
            Richtung::Unten => neuer_kopf.y += 1,
            Richtung::Links => neuer_kopf.x -= 1,
            Richtung::Rechts => neuer_kopf.x += 1,
        }
        self.koerper.insert(0, neuer_kopf);
        self.koerper.pop();
    }
    fn wachse(&mut self) {
        let letzter = self.koerper.last().unwrap().clone();
        self.koerper.push(letzter);
    }
    fn kollidiert_mит_себя(&self) -> bool {
        let kopf = &self.koerper[0];
        self.koerper[1..].iter().any(|pos| pos.x == kopf.x && pos.y == kopf.y)
    }
}

struct Apfel {
    position: Position,
}

impl Apfel {
    fn neuer() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            position: Position {
                x: rng.gen_range(0..(FENSTER_GROESSE.0 / BLOCK_GROESSE) as i32),
                y: rng.gen_range(0..(FENSTER_GROESSE.1 / BLOCK_GROESSE) as i32),
            },
        }
    }
}

fn main() {
    let mut fenster: PistonWindow = WindowSettings::new("Schlange", FENSTER_GROESSE)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut schlange = Schlange::neue();
    let mut apfel = Apfel::neuer();
    let mut punkte = 0;
    let mut spiel_zu_ende = false;

    let mut timer = 0.0;
    let geschwindigkeit = 0.15; // Sekunden zwischen den Schritten

    while let Some(event) = fenster.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if !spiel_zu_ende {
                schlange.richtung = match key {
                    Key::W if schlange.richtung != Richtung::Unten => Richtung::Oben,
                    Key::S if schlange.richtung != Richtung::Oben => Richtung::Unten,
                    Key::A if schlange.richtung != Richtung::Rechts => Richtung::Links,
                    Key::D if schlange.richtung != Richtung::Links => Richtung::Rechts,
                    _ => schlange.richtung.clone(),
                };
            }
        }

        if let Some(args) = event.update_args() {
            timer += args.dt;
            if !spiel_zu_ende && timer >= geschwindigkeit {
                timer = 0.0;
                schlange.bewege();

                // Überprüfung auf Kollision mit dem Rand
                let kopf = &schlange.koerper[0];
                if kopf.x < 0
                    || kopf.y < 0
                    || kopf.x >= (FENSTER_GROESSE.0 / BLOCK_GROESSE) as i32
                    || kopf.y >= (FENSTER_GROESSE.1 / BLOCK_GROESSE) as i32
                {
                    spiel_zu_ende = true;
                }

                if schlange.koerper[0].x == apfel.position.x && schlange.koerper[0].y == apfel.position.y {
                    schlange.wachse();
                    apfel = Apfel::neuer();
                    punkte += 1;
                }
                if schlange.kollidiert_mит_себя() {
                    spiel_zu_ende = true;
                }
            }
        }

        fenster.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            if !spiel_zu_ende {
                // Zeichnen der Schlange
                for pos in &schlange.koerper {
                    rectangle(
                        [0.0, 1.0, 0.0, 1.0], // grüne Schlange
                        [
                            (pos.x as f64) * BLOCK_GROESSE as f64,
                            (pos.y as f64) * BLOCK_GROESSE as f64,
                            BLOCK_GROESSE as f64,
                            BLOCK_GROESSE as f64,
                        ],
                        c.transform,
                        g,
                    );
                }
                // Zeichnen des Apfels
                rectangle(
                    [1.0, 0.0, 0.0, 1.0], // roter Apfel
                    [
                        (apfel.position.x as f64) * BLOCK_GROESSE as f64,
                        (apfel.position.y as f64) * BLOCK_GROESSE as f64,
                        BLOCK_GROESSE as f64,
                        BLOCK_GROESSE as f64,
                    ],
                    c.transform,
                    g,
                );
            }
        });
    }
}