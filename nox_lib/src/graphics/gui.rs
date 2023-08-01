use std::io::prelude::*;

use super::Color;

// each character is in the format of
// | foreground color   |  | background color   | | character | | clear ansii settings |
// \x1b[38;2;rrr;ggg;bbbm  \x1b[48;2;rrr;ggg;bbbm       c             \x1b[m

// all together
// \x1b[38;2;rrr;ggg;bbbm\x1b[48;2;rrr;ggg;bbbmc\x1b[m


pub struct Gui {
    width: i64,
    height: i64,

    foreground_color_buffer: Vec<Vec<Color>>,
    background_color_buffer: Vec<Vec<Color>>,
    character_buffer: Vec<Vec<char>>,
    print_buffer: Vec<char>
}

impl Gui {
    pub fn new(width: i64, height: i64) -> Self {
        let width = width as usize; // the reason i64 is used is to limit the size of the gui and to allow the line alg to use negative numbers
        let height = height as usize;

        let mut foreground_color_buffer = Vec::with_capacity(height);
        let mut background_color_buffer = Vec::with_capacity(height);
        let mut character_buffer = Vec::with_capacity(height);

        let mut color_buffer = Vec::with_capacity(width);
        for _ in 0..width {
            color_buffer.push(Color::new(0, 0, 0));
        }

        let mut line_buffer = Vec::with_capacity(width);
        for _ in 0..width {
            line_buffer.push(' ');
        }

        for _ in 0..height {
            foreground_color_buffer.push(color_buffer.clone());
            background_color_buffer.push(color_buffer.clone());
            character_buffer.push(line_buffer.clone());
        }

        let pixel_count = width * height;
        let new_line_count = width;
        let ansi_code_count = 39 * pixel_count + 3;

        let mut print_buffer = Vec::with_capacity(pixel_count + new_line_count + ansi_code_count); // this is a temp size, need to recalculate for the ansi escape codes

        for _ in 0..print_buffer.capacity() {
            print_buffer.push(' ');
        }

        let mut index = 0;
        for _ in 0..height {
            for _ in 0..width { // for every pixel in the screen
                print_buffer[index] = '\x1b';      index += 1;
                print_buffer[index] = '[';         index += 1;
                print_buffer[index] = '3';         index += 1;
                print_buffer[index] = '8';         index += 1;
                print_buffer[index] = ';';         index += 1;
                print_buffer[index] = '2';         index += 1;
                print_buffer[index] = ';';         index += 4;
                print_buffer[index] = ';';         index += 4;
                print_buffer[index] = ';';         index += 4;
                print_buffer[index] = 'm';         index += 1;
                print_buffer[index] = '\x1b';      index += 1;
                print_buffer[index] = '[';         index += 1;
                print_buffer[index] = '4';         index += 1;
                print_buffer[index] = '8';         index += 1;
                print_buffer[index] = ';';         index += 1;
                print_buffer[index] = '2';         index += 1;
                print_buffer[index] = ';';         index += 4;
                print_buffer[index] = ';';         index += 4;
                print_buffer[index] = ';';         index += 4;
                print_buffer[index] = 'm';         index += 2;
            }

            print_buffer[index] = '\n';   index += 1;
        }

        print_buffer[index] = '\x1b'; index += 1;
        print_buffer[index] = '[';    index += 1;
        print_buffer[index] = 'm';

        let width = width as i64;
        let height = height as i64;

        Self {
            width,
            height,
            foreground_color_buffer,
            background_color_buffer,
            character_buffer,
            print_buffer
        }
    }

    pub fn display(&mut self) {
        let mut index = 0;

        for y in 0..self.height as usize {
            for x in 0..self.width as usize { // for every pixel in the screen
                
                // Get the character as well as the foreground and background colors
                let c = self.character_buffer[y][x];
                let fg = self.foreground_color_buffer[y][x];
                let bg = self.background_color_buffer[y][x];

                // Get the rgb components of the foreground and background colors
                let fg_red = format!("{:0>3}", fg.r).chars().collect::<Vec<_>>();
                let fg_green = format!("{:0>3}", fg.g).chars().collect::<Vec<_>>();
                let fg_blue = format!("{:0>3}", fg.b).chars().collect::<Vec<_>>();

                let bg_red = format!("{:0>3}", bg.r).chars().collect::<Vec<_>>();
                let bg_green = format!("{:0>3}", bg.g).chars().collect::<Vec<_>>();
                let bg_blue = format!("{:0>3}", bg.b).chars().collect::<Vec<_>>();

                // Set the elements in the print buffer to the required characters
                index += 7;
                self.print_buffer[index] =   fg_red[0]; index += 1; // fg red
                self.print_buffer[index] =   fg_red[1]; index += 1;
                self.print_buffer[index] =   fg_red[2]; index += 2;
                self.print_buffer[index] = fg_green[0]; index += 1; // fg green
                self.print_buffer[index] = fg_green[1]; index += 1;
                self.print_buffer[index] = fg_green[2]; index += 2;
                self.print_buffer[index] =  fg_blue[0]; index += 1; // fg blue
                self.print_buffer[index] =  fg_blue[1]; index += 1;
                self.print_buffer[index] =  fg_blue[2]; index += 9;
                self.print_buffer[index] =   bg_red[0]; index += 1; // bg red
                self.print_buffer[index] =   bg_red[1]; index += 1;
                self.print_buffer[index] =   bg_red[2]; index += 2;
                self.print_buffer[index] = bg_green[0]; index += 1; // bg green
                self.print_buffer[index] = bg_green[1]; index += 1;
                self.print_buffer[index] = bg_green[2]; index += 2;
                self.print_buffer[index] =  bg_blue[0]; index += 1; // bg blue
                self.print_buffer[index] =  bg_blue[1]; index += 1;
                self.print_buffer[index] =  bg_blue[2]; index += 2;
                self.print_buffer[index] =           c; index += 1; // character
            }
            index += 1;
        }

        // turn the print buffer into a string
        let mut print_string = self.print_buffer.iter().collect::<String>();

        // trim excess spaces, dont know why they are inserted, probably something to do with utf-8
        print_string = print_string.trim_end_matches(" ").to_string();

        // print the string to stdout
        print!("\x1b[H{}", print_string);
        std::io::stdout().flush().unwrap();
    }
}

impl Gui {
    pub fn width(&self) -> &i64 {
        &self.width
    }

    pub fn height(&self) -> &i64 {
        &self.height
    }
}

impl Gui { // Helpers
    pub fn clear_terminal(&self) {
        print!("\x1b[H\x1b[2J");
    }

    pub fn hide_cursor(&self) {
        print!("\x1b[?25l");
        std::io::stdout().flush().unwrap();
    }

    pub fn show_cursor(&self) {
        print!("\x1b[?25h");
        std::io::stdout().flush().unwrap();
    }
}

impl Gui { // Drawing Functions
    pub fn pixel(&mut self, x: i64, y: i64, c: char, fg: Color, bg: Color) {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        self.character_buffer[y][x] = c;

        if fg != Color::DEFAULT {
            self.foreground_color_buffer[y][x] = fg;
        }
    
        if bg != Color::DEFAULT {
            self.background_color_buffer[y][x] = bg;
        }
    }

    pub fn clear(&mut self, c: char, fg: Color, bg: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.pixel(x, y, c, fg, bg);
            }
        }
    }

    pub fn stroke_rect(&mut self, x: i64, y: i64, width: i64, height: i64, c: char, fg: Color, bg: Color) {
        for x in x..x+width {
            self.pixel(x, y, c, fg, bg);
            self.pixel(x, y+height-1, c, fg, bg);
        }

        for y in y..y+height {
            self.pixel(x, y, c, fg, bg);
            self.pixel(x+width-1, y-1, c, fg, bg);
        }
    }

    pub fn fill_rect(&mut self, x: i64, y: i64, width: i64, height: i64, c: char, fg: Color, bg: Color) {
        for y in y..y+height {
            for x in x..x+width {
                self.pixel(x, y, c, fg, bg);
            }
        }
    }

    pub fn rect(&mut self, x: i64, y: i64, width: i64, height: i64, stroke_char: char, fill_char: char, stroke_fg: Color, stroke_bg: Color, fill_fg: Color, fill_bg: Color) {
        for y_coord in 0..height {
            for x_coord in 0..width {
                if x_coord == 0 || x_coord == width - 1 || y_coord == 0 || y_coord == height - 1 {
                    self.pixel(x + x_coord, y + y_coord, stroke_char, stroke_fg, stroke_bg);
                }
                else {
                    self.pixel(x + x_coord, y + y_coord, fill_char, fill_fg, fill_bg);
                }
            }
        }
    }

    pub fn horizontal_text(&mut self, x: i64, y: i64, text: &str, fg: Color, bg: Color) {
        for i in 0..text.len() as i64 {
            self.pixel(x + i, y, text.chars().collect::<Vec<_>>()[i as usize], fg, bg);
        }
    }

    pub fn vertical_text(&mut self, x: i64, y: i64, text: &str, fg: Color, bg: Color) {
        for i in 0..text.len() as i64 {
            self.pixel(x, y + i, text.chars().collect::<Vec<_>>()[i as usize], fg, bg);
        }
    }

    pub fn line(&mut self, mut x0: i64, mut y0: i64, mut x1: i64, mut y1: i64, c: char, fg: Color, bg: Color) { // bresenham's line alg
        let mut dx = x1 - x0;
        let mut dy = y1 - y0;

        if dy == 0 { // Horizontal
            if x0 > x1 {
                let temp = x0;
                x0 = x1;
                x1 = temp;
                y0 = y1;
            }

            for x in x0..=x1 {
                self.pixel(x, y0, c, fg, bg);
            }

            return;
        }
        else if dx == 0 { // Vertical
            if y0 > y1 {
                x0 = x1;
                let temp = y0;
                y0 = y1;
                y1 = temp;
            }

            for y in y0..=y1 {
                self.pixel(x0, y, c, fg, bg);
            }

            return;
        }
        
        if x0 > x1 { // move everything to the 4 right octants (dx is always positive)
            let mut temp = x0;
            x0 = x1;
            x1 = temp;
            temp = y0;
            y0 = y1;
            y1 = temp;

            dx = x1 - x0;
            dy = y1 - y0;
        }

        let loop_x = dy.abs() <= dx.abs();

        if loop_x {
            if x0 > x1 {
                let mut temp = x0;
                x0 = x1;
                x1 = temp;
                temp = y0;
                y0 = y1;
                y1 = temp;

                dx = x1 - x0;
                dy = y1 - y0;
            }

            let res_mod = if dy > 0 {1} else {-1};
            let mut y = y0;
            let mut err = -dx;
            let slope = dy;
            let dx2 = dx << 1; // bitshift instead of multiply by 2

            for x in x0..=x1 {
                self.pixel(x, y, c, fg, bg);
                err += 2 * slope.abs();

                if err >= 0 {
                    err -= dx2;
                    y += res_mod;
                }
            }
        }
        else {
            if y0 > y1 {
                let mut temp = x0;
                x0 = x1;
                x1 = temp;
                temp = y0;
                y0 = y1;
                y1 = temp;

                dx = x1 - x0;
                dy = y1 - y0;
            }

            let res_mod = if dx > 0 {1} else {-1};
            let mut x = x0;
            let mut err = -dy;
            let slope = dx;
            let dy2 = dy << 1; // bitshift instead of multiply by 2

            for y in y0..=y1 {
                self.pixel(x, y, c, fg, bg);
                err += 2 * slope.abs();

                if err >= 0 {
                    err -= dy2;
                    x += res_mod;
                }
            }
        }
        
   
    }
}

// let dx = x1 - x0;
// let dy = y1 - y0;

// let mut y = y0;
// let slope = 2 * dy;
// let mut err = -dx;
// let err_inc = -2 * dx;

// for x in x0..=x1 {
//     self.pixel(x, y, c, fg, bg);
//     err += slope;

//     if err >= 0 {
//         err += err_inc;
//         y += 1;
//     }
// }