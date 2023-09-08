use std::{
    ops::DerefMut,
    rc::Rc,
    cell::RefCell,
    sync::{
        Arc,
        Mutex
    }
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
use ratatui::{
    style::Color,
    widgets::{
        canvas::{Painter, Shape}},
};
const GRAY_SCALE: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
#[derive(Clone, Copy)]
pub enum CamMode {
    Pixels,
    Char,
    ColorChar
}
pub const TOTAL_VARIANTS : i32= 3;
pub struct CameraPic{
    pub cam : Arc<Mutex<VideoCapture>>,
    pub  width : u16,
    pub height : u16
}
impl CameraPic{
    pub fn new() -> Self {
    let mut dev = VideoCapture::new(0, videoio::CAP_ANY)
        .expect("impossible to open camera\n");

	let opened = videoio::VideoCapture::is_opened(&dev)
        .expect("camera isn't open");

    //let mut frame = Mat::default();
    //dev.read(&mut frame).unwrap();
    //print!("{:?}",frame);
    //let flags : Vector<i32>= Vector::from_iter(vec![]);
    //imgcodecs::imwrite("./test.png", &frame, &flags).unwrap();
    CameraPic{
        cam : Arc::new(Mutex::new(dev)),
        height : 0,
        width : 0
    }
    }
}
pub fn rgb2ascii(r : f32, g : f32, b : f32) -> char{
    let gray = (0.299*r + 0.587*g + 0.114*b);
    GRAY_SCALE.chars().nth((GRAY_SCALE.len() as f32 - (gray*(GRAY_SCALE.len() as f32)/255.0) - 1.0) as usize).unwrap()
}
impl Shape for CameraPic {

    fn draw(&self, painter: &mut Painter) {
        let mut frame = Mat::default();
        let mut resize_frame = Mat::default();
        //let dev : &mut VideoCapture = self.cam.try_lock().unwrap().deref_mut();
            //Rc::try_unwrap(self.cam.clone())
            //.unwrap()
            //.into_inner();
        self.cam.try_lock().unwrap().deref_mut().read(&mut frame).unwrap();
        opencv::imgproc::resize(&frame, &mut resize_frame, Size_ { width: (self.width as i32)*2, height: (self.height as i32)*4 }, 0.0, 0.0, 0).unwrap();
        //print!("{:?}\n",frame);
        for x in 0..((self.width-2)*2) as usize{
            for y in 0..((self.height)*4) as usize{
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
                painter.paint(x, y, Color::Rgb(color_r, color_g, color_b));
            }
        }

    }
    
}
