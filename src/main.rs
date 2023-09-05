mod camera;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, ops::Deref};
use ratatui::{
    terminal::{
        Terminal,
        Frame
    },
    widgets::{block::Block, Borders, List, ListItem, canvas::{Canvas, Map, Painter, Points, Shape}},
    backend::{Backend, CrosstermBackend}, 
    prelude::{Layout, Constraint, Direction}, 
    text::{Line, Text, self, Span},
    style::{
        Style,
        Color
    }
};
use crate::camera::CameraPic;

struct Picture{
    color : Color,
    width : u16,
    height : u16
}

impl Shape for Picture{

    fn draw(&self, painter: &mut Painter) {
        for x in 0..10{
            for y in 0..10{
                painter.paint(x, y, Color::Blue);
            }
        }
        painter.paint(0, 0, Color::Blue);
        for x in 0..((self.width)*2) as usize{
            for y in 0..((self.height)*4) as usize{
                painter.paint(x, y, Color::Red);
            }
        }

    }
}

fn main() {
    //enter raw mode for the terminal
    let mut cam = CameraPic::new();
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    //enter alternate buffer for terminal
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    //create backend
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    loop{
        terminal.draw(|f| draw_sheet(f,&mut cam)).unwrap();
    }
    
    disable_raw_mode().unwrap();
    //exit alternate buffer for terminal
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();
}
fn draw_sheet<B: Backend>(f: &mut Frame<B>, cam : &mut CameraPic){
   let chonkers = Layout::default()
       .constraints([Constraint::Length(20), Constraint::Ratio(16, 9)])
       .direction(Direction::Horizontal)
       .split(f.size());
   cam.width = chonkers[1].width;
   cam.height = chonkers[1].height;
   let items : Vec<&str> = vec!["Ciao", "Ciao", "No"];

   let list_items : Vec<ListItem> = 
       items
       .iter()
       .map(
           |str| ListItem::new(vec![text::Line::from(Span::raw(*str))])
           )
       .collect();

   let selector = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("Test1"))
        .highlight_style(Style::default().fg(Color::Yellow));
   f.render_widget(selector, chonkers[0]);

   let map = Canvas::default()
       .block(Block::default().title("World").borders(Borders::ALL))
       .paint(|ctx| {
           //ctx.draw(&Map {
           //    color: Color::White,
           //     resolution: ratatui::widgets::canvas::MapResolution::High,
           // });
           //ctx.layer();
           //for x in 0..chonkers[1].width {
           //    for y in 0..chonkers[1].height{
           //        ctx.draw(&Points{
           //     coords: &[(x as f64, y as f64)],
           //     color: Color::Red

           //})
           // }
           //}
           ctx.draw(cam);
           ctx.layer();
            ctx.print(0.0, 2.0, "ciao");
            ctx.print(chonkers[1].width as f64, 0.0, "c");
       })
        .marker(ratatui::symbols::Marker::Braille)
        .x_bounds([0.0, chonkers[1].width as f64])
        .y_bounds([0.0, chonkers[1].height as f64]);
   f.render_widget(map, chonkers[1])
}
