use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

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
        let width = 64;
        let height = 64;

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

// use std::fmt;

// impl fmt::Display for Universe {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for line in self.cells.as_slice().chunks(self.width as usize) {
//             for &cell in line {
//                 let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
//                 write!(f, "{}", symbol)?;
//             }
//             write!(f, "\n")?;
//         }

//         Ok(())
//     }
// }

fn draw(game: &mut Universe) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    game.tick();
    const CELL_SIZE: f64 = 12.0;

    draw_grid(&game, &ctx, CELL_SIZE);
    draw_cells(&game, &ctx, CELL_SIZE);
}

fn draw_grid(game: &Universe, ctx: &web_sys::CanvasRenderingContext2d, CELL_SIZE: f64) {
    const GRID_COLOR: &str = "#CCCCCC";

    ctx.begin_path();
    // Vertical lines.
    for i in 0..=game.height{
        let i = i as f64;
        ctx.move_to(i * (CELL_SIZE + 1.0) + 1.0, 0.0);
        ctx.line_to(i * (CELL_SIZE + 1.0) + 1.0, (CELL_SIZE + 1.0) * game.height as f64 + 1.0);
    }

    // Horizontal lines.
    for j in 0..=game.width as usize{
        let j = j as f64;
        ctx.move_to(0.0,                           j * (CELL_SIZE + 1.0) + 1.0);
        ctx.line_to((CELL_SIZE + 1.0) * game.width as f64 + 1.0, j * (CELL_SIZE + 1.0) + 1.0);
    }

    ctx.stroke();
}

fn draw_cells(game: &Universe, ctx: &web_sys::CanvasRenderingContext2d, CELL_SIZE: f64) {
    const DEAD_COLOR: &str = "#FFFFFF";
    const ALIVE_COLOR: &str = "#000000";

    ctx.begin_path();
    for i in 0..game.height as usize {
        for j in 0..game.width as usize{
            let idx = game.get_index(i as u32, j as u32);
            
            ctx.set_fill_style(&JsValue::from_str(match game.cells[idx] {
                Cell::Dead => DEAD_COLOR,
                Cell::Alive => ALIVE_COLOR,
            }));

            ctx.fill_rect(
                i as f64 * (CELL_SIZE + 1.0) + 1.0,
                j as f64 * (CELL_SIZE + 1.0) + 1.0,
                CELL_SIZE,
                CELL_SIZE,
            );
        }
    }
    ctx.stroke();
}

fn app(cx: Scope) -> Element {
    let game = use_ref(cx, Universe::new);

    // Set up an interval to call draw(game) every 100 milliseconds
    let game_clone = game.clone();
    gloo_timers::callback::Interval::new(250, move || game_clone.with_mut(|game| draw(game))).forget();

    cx.render(rsx! {
        div {
            canvas {
                id: "canvas",
                width: "1000",
                height: "1000",
            }
            // game1.with(|game1| format!("{}", game1))
            // button {
            //     onclick: move |_| game.with_mut(|game| draw(game)),
            //     "Click me!"
            // }
            // button {
            //     onclick: move |_| game.with_mut(|game| for _ in 0..10 {draw(game)}),
            //     "Click me!"
            // }
        }
    })
}
