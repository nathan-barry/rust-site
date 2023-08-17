use std::str::FromStr;
use std::rc::Rc;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, console};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

pub struct Game {
    size: u32,
    cells: Vec<Cell>,
    // document: Document,
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    cell_size: f64,
    alive_color: String,
    dead_color: String,
    grid_color: String,
}

impl Game {
    fn new() -> Game {
        let cell_size: f64 = 5.0;
        let alive_color = String::from_str("#d4d4d4").unwrap();
        let dead_color = String::from_str("#191919").unwrap();
        let grid_color = String::from_str("#7f7f7f").unwrap();

        let window = web_sys::window().unwrap();
        let inner_size = window.inner_width().unwrap().as_f64().unwrap();

        // You can set canvas and grid dimensions based on the viewport
        let canvas_size = if inner_size > (1000.0) { 901 } else {(inner_size * 0.90) as u32};

        let size = canvas_size / (cell_size+1.0) as u32; // Adjusting number of columns based on cell size

        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into()
            .map_err(|_| ())
            .unwrap();
        let context: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        canvas.set_width(canvas_size);
        canvas.set_height(canvas_size);

        let cells: Vec<Cell> = (0..size * size)
            .map(|i| {
                if i % 12 == 0  || i % 13 == 0 || i % 17 == 0 || i % 19 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }).collect();

        Game {
            size,
            cells,
            canvas,
            context,
            cell_size,
            alive_color,
            dead_color,
            grid_color,
        }
    }


    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.size + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.size - 1, 0, 1].iter().cloned() {
            for delta_col in [self.size - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.size;
                let neighbor_col = (col + delta_col) % self.size;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.size{
            for col in 0..self.size{
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

    fn draw_grid(&self) {
        let ctx = &self.context;
        let cell_size = self.cell_size;

        ctx.set_stroke_style(&JsValue::from_str(&self.grid_color));

        ctx.begin_path();
        // Vertical lines.
        for i in 0..=self.size{
            let i = i as f64;
            ctx.move_to(i * (cell_size + 1.0) + 1.0, 0.0);
            ctx.line_to(i * (cell_size + 1.0) + 1.0, (cell_size + 1.0) * self.size as f64 + 1.0);
        }

        // Horizontal lines.
        for j in 0..=self.size as usize{
            let j = j as f64;
            ctx.move_to(0.0,                           j * (cell_size + 1.0) + 1.0);
            ctx.line_to((cell_size + 1.0) * self.size as f64 + 1.0, j * (cell_size + 1.0) + 1.0);
        }

        ctx.stroke();
    }

    fn draw_cells(&self) {
        let ctx = &self.context;
        let cell_size = self.cell_size;

        ctx.begin_path();
        for i in 0..self.size as usize {
            for j in 0..self.size as usize{
                let idx = self.get_index(i as u32, j as u32);
                
                ctx.set_fill_style(&JsValue::from_str(match self.cells[idx] {
                    Cell::Dead => &self.dead_color,
                    Cell::Alive => &self.alive_color,
                }));

                ctx.fill_rect(
                    i as f64 * (cell_size + 1.0) + 1.0,
                    j as f64 * (cell_size + 1.0) + 1.0,
                    cell_size,
                    cell_size,
                );
            }
        }
        ctx.stroke();
    }

    fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
}

fn draw(game: Rc<RefCell<Game>>) {
    let mut game = game.borrow_mut();
    game.tick();
    game.draw_cells();
}

fn draw_handler(game: Rc<RefCell<Game>>) {
    let game_ref = game.clone();
    let canvas_ref = game.borrow().canvas.clone();
    let cell_size_ref = game.borrow().cell_size;
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let mut game = game_ref.borrow_mut();

        // Calculate the click's x and y coordinates relative to the canvas
        let bounding_rect = canvas_ref.get_bounding_client_rect();
        let scale_x = canvas_ref.width() as f64 / bounding_rect.width();
        let scale_y = canvas_ref.height() as f64 / bounding_rect.height();
        let canvas_x = (event.client_x() as f64 - bounding_rect.left()) * scale_x;
        let canvas_y = (event.client_y() as f64 - bounding_rect.top()) * scale_y;

        // Calculate the row and column of the clicked cell
        let row = (canvas_x / (cell_size_ref + 1.0)) as u32;
        let col = (canvas_y / (cell_size_ref + 1.0)) as u32;

        let row = match row {
            row if row == 0 => 1,
            row if row == game.size-1 => game.size-2,
            row => row,
        };
        let col = match col {
            col if col == 0 => 1,
            col if col == game.size-1 => game.size-2,
            col => col,
        };

        // Different patterns
        if row % 2 == 0 {
            game.toggle_cell(row+1, col);
            game.toggle_cell(row-1, col-1);
            game.toggle_cell(row-1, col);
            game.toggle_cell(row-1, col+1);
            game.toggle_cell(row, col+1);
        } else {
            game.toggle_cell(row-1, col);
            game.toggle_cell(row+1, col+1);
            game.toggle_cell(row+1, col);
            game.toggle_cell(row+1, col-1);
            game.toggle_cell(row, col-1);
        }
        game.draw_cells();
    }) as Box<dyn FnMut(_)>);

    game.borrow().canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

#[inline_props]
pub fn GameOfLife(cx: Scope) -> Element {
    let interval = Rc::new(RefCell::new(Interval::new(None)));
    let interval_clone = interval.clone();

    cx.render(rsx! {
        p { class: "mb-4",
            "Below is Conway's Game of Life, implemented in Rust and rendered on the front end via WebAssembly."
        }
        p { class: "mb-8 md:mb-16",
            "Click on a square to make it alive."
        }
        div {
            class: "mb-8 w-full flex justify-center",
            canvas {
                id: "canvas",
                onmounted: move |_| {
                    console::log_1(&JsValue::from_str("Game of Life Page Mounted"));
                    let game = Rc::new(RefCell::new(Game::new()));
                    game.borrow_mut().draw_grid();
                    draw_handler(game.clone());
                    // Set up an interval to call draw(game) every 100 milliseconds
                    let new_interval = gloo_timers::callback::Interval::new(50, move || draw(game.clone()));
                    interval_clone.borrow_mut().interval = Some(new_interval);
                }
            }
        }
    })
}

struct Interval {
    interval: Option<gloo_timers::callback::Interval>,
}

impl Interval {
    pub fn new(interval: Option<gloo_timers::callback::Interval>) -> Self {
        Interval {
            interval
        }
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        if let Some(i) = self.interval.take() {
            i.cancel();
        }
        console::log_1(&JsValue::from_str("Game of Life Page Closed"));
    }
}
