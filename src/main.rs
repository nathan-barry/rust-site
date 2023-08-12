use std::f64;
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn new() -> Universe {
        let width = 16;
        let height = 16;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }).collect();

        Universe {
            width,
            height,
            cells,
        }
    }


    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height{
            for col in 0..self.width{
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Dies from underpopulation
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Lives
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Dies from overpopulation
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Dead cell w/ 3 neighbors comes alive
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in same state
                    (otherwise, _) => otherwise
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
}


use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

// fn draw() {
//     let document = web_sys::window().unwrap().document().unwrap();
//     let canvas = document.get_element_by_id("canvas").unwrap();
//     let canvas: web_sys::HtmlCanvasElement = canvas
//         .dyn_into::<web_sys::HtmlCanvasElement>()
//         .map_err(|_| ())
//         .unwrap();

//     let context = canvas
//         .get_context("2d")
//         .unwrap()
//         .unwrap()
//         .dyn_into::<web_sys::CanvasRenderingContext2d>()
//         .unwrap();

//     context.begin_path();

//     // Draw the outer circle.
//     context
//         .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
//         .unwrap();

//     // Draw the mouth.
//     context.move_to(110.0, 75.0);
//     context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

//     // Draw the left eye.
//     context.move_to(65.0, 65.0);
//     context
//         .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
//         .unwrap();

//     // Draw the right eye.
//     context.move_to(95.0, 65.0);
//     context
//         .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
//         .unwrap();

//     context.stroke();
// }

fn app(cx: Scope) -> Element {
    // use_effect(cx, (), |_| {
    //     async move {
    //         canvas.height
    //         draw();
    //     }
    // });
    let game1 = use_ref(cx, || Universe::new());

    cx.render(rsx! {
        div {
            // canvas {
            //     id: "canvas",
            //     width: "500",
            //     height: "400",
            //     style: "border: 1px solid black",
            // }
            game1.with(|game1| format!("{}", game1))
            button {
                onclick: move |_| game1.with_mut(|game1| game1.tick()),
                "Click me!"
            }
        }
    })
}
