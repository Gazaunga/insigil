
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use std::collections::VecDeque;

use state;
use state::{RingDataBuffer, RingDataBufferType, RingData};
use std::cmp::{min,max};
use opengl_graphics::{ GlGraphics, OpenGL }; 
use graphics::*;

use time;
use time::Tm;
use widget::Widget;

pub struct HistoRing {
    sliding: bool,
    targetTmMs: Tm,
    size: f64,
    x: f64,
    y: f64,
    id: i32,
    dat: RingDataBuffer
}

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const GREEN_05: [f32; 4] = [0.0, 1.0, 0.0, 0.5];

impl HistoRing {
    pub fn new(x: f64, y: f64, size: f64, id: i32, dat: state::RingDataBuffer) -> HistoRing {
        HistoRing { 
            sliding: false,
            targetTmMs: time::now(),
            size: size,
            x: x, y: y,
            id: id, dat: dat
         }
    }
}

impl<G> Widget<G> for HistoRing
    where G: Graphics{
    fn draw(
        &mut self,
        //ringbounds: [f64; 4],
        transform: math::Matrix2d,
        g: &mut G,
        //size: f64
        ) where G: Graphics {
    let radius = self.size * 0.5;
    let buffer = 5.0;
    let mut rdbints = VecDeque::<i32>::new();
    let ref mut dat = self.dat;

    //calculate stuff
    let (sum,max,avg) = {
            let sum: i32 = rdbints.iter().sum();
            let max = rdbints.iter().fold(0,|largest, &i| max(i, largest));
            let avg: f32 = sum as f32 / rdbints.len() as f32;
            //print!("\rs,m,a: {:?} {:?} {:?}", sum, max, avg);
            (sum,max,avg)
    };
    let working = (radius - buffer - (radius - self.size)) as f64;
    let scale = working / max as f64;

    let ringbounds=rectangle::centered_square
        (self.x,
        self.y,
        self.size / 2.0);
    //draw stuff
    //rectangle(GREEN,[0.0,-10.0,10.0,10.0], transform, g);
    circle_arc(GREEN_05, 1.0, 0.0, 6.282, ringbounds, transform, g);
    match *dat {
        RingDataBuffer::Ints(ref intsq) => {
            for (idx, i) in intsq.iter().enumerate() {
                //println!("draw {:?}", i.clone());
                let t = transform.rot_rad(0.031415 * idx as f64);
                let line = rectangle::centered(
                    [0.0, 0.0, (radius - buffer),
                     (radius - buffer) - (i.clone() as f64 * scale)]
                                                   );
                rectangle(GREEN_05, line, transform, g);
            }
        },
        RingDataBuffer::Text(ref text) => {
            println!("{:?}", text);
        }
        RingDataBuffer::DatedInts(ref dis) => {
            println!("{:?}", dis);
        }

    }
        

    }
    fn getid(&mut self) -> i32 { self.id }
    fn push (
        &mut self,
        rdata: state::RingData
        ) {
        match rdata {
          RingData::Int(i) => { 
            match self.dat {
              RingDataBuffer::Ints(ref mut intq) => { intq.push_front(i) ;
                                                      println!("pushed: {:?}", i)
                                                      },
              _ => {}
            }
          },
          RingData::Text(s) => {},
          RingData::Date(d) => {},
          }

        /*match self.dat {
          RingDataBuffer::Ints(ref intsq) => { intsq.push_front(rdata.Int)},
          RingDataBuffer::Text(ref text) => {},
          RingDataBuffer::DatedInts(ref dis) => {},
        }
        */

    }


}
