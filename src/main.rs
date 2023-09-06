mod camera;

use std::{
    ops::DerefMut,
    rc::Rc,
    cell::RefCell,
    sync::{
        Arc,
        Mutex
    }
};
use camera::rgb2ascii;
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
use crate::camera::{
    CameraPic,
    CamMode
};
use opencv::{
    core::{
        Vector,
        Size_
    },
    prelude::{
        MatTraitConst,
        VideoCaptureTrait,
        VideoCaptureTraitConst,
        Mat
    },
    imgcodecs::{
        imdecode,
        ImreadModes,
        self
    },
    videoio::{
        VideoCapture, self
    }
};

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
struct TermState{
    pub cam : CameraPic,
    selected_mode : CamMode,
}
impl TermState {
    fn new(cam : CameraPic) -> Self{
        TermState { cam: cam, selected_mode: CamMode::Pixels }
    }
    fn mode_up(&mut self){
        self.selected_mode = match self.selected_mode{
            CamMode::Char => CamMode::Pixels,
            CamMode::Pixels => CamMode::Char
        }
    }

    fn mode_down(&mut self){
        self.selected_mode = match self.selected_mode{
            CamMode::Char => CamMode::Pixels,
            CamMode::Pixels => CamMode::Char
        }
    }
}


fn main() {
    //enter raw mode for the terminal
    let cam = CameraPic::new();
    let mut state = TermState::new(cam);
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    //enter alternate buffer for terminal
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    //create backend
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    'drawloop: loop{
        terminal.draw(|f| draw_sheet(f,&mut state)).unwrap();
        if crossterm::event::poll(core::time::Duration::from_millis(16)).unwrap(){
            if let Event::Key(key) = crossterm::event::read().unwrap(){
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('q') => {break 'drawloop;},
                        crossterm::event::KeyCode::Char('j') => state.mode_down(),
                        crossterm::event::KeyCode::Char('k') => state.mode_up(),
                        _ => {}
                    }
                }
            }
        }
    }
    
    disable_raw_mode().unwrap();
    //exit alternate buffer for terminal
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();
}

fn draw_sheet<B: Backend>(f: &mut Frame<B>, state : &mut TermState){
   let chonkers = Layout::default()
       .constraints([Constraint::Length(20), Constraint::Ratio(16, 9)])
       .direction(Direction::Horizontal)
       .split(f.size());

   state.cam.width = chonkers[1].width;
   state.cam.height = chonkers[1].height;

   let items : Vec<&str> = vec!["Characters", "Pixels"];

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
           match state.selected_mode {
               CamMode::Pixels => {
                   ctx.draw(&state.cam);
               }
               CamMode::Char => {
                   let mut frame = Mat::default();
                   let mut resize_frame = Mat::default();
                   state.cam.cam.try_lock().unwrap().deref_mut().read(&mut frame).unwrap();
                   opencv::imgproc::resize(&frame, &mut resize_frame, Size_ { width: (state.cam.width as i32), height: (state.cam.height as i32) }, 0.0, 0.0, 0).unwrap();
                   for x in 0..(state.cam.width-2) as usize{
                       for y in 0..(state.cam.height) as usize{
                           let point = resize_frame
                               .ptr_2d(y as i32, x as i32)
                               .unwrap();
                           let color_b : u8 = unsafe{
                               point
                                   .add(0)
                                   .read()
                           };
                           let color_g :u8 = unsafe{
                               point
                                   .add(1)
                                   .read()
                           };
                           let color_r : u8 = unsafe{
                               point
                                   .add(2)
                                   .read()
                           };
                           ctx.print(x as f64, state.cam.height as f64 - y as f64, String::from(rgb2ascii(color_r as f32, color_g as f32, color_b as f32)));

                       }
                   }
               }
           }
           //ctx.layer();
           // ctx.print(0.0, 2.0, "ciao");
           // ctx.print(chonkers[1].width as f64, 0.0, "c");
       })
        .marker(ratatui::symbols::Marker::Braille)
        .x_bounds([0.0, chonkers[1].width as f64])
        .y_bounds([0.0, chonkers[1].height as f64]);
   f.render_widget(map, chonkers[1])
}
