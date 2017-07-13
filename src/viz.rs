
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use std::collections::VecDeque;

use state;
use state::{RingDataBuffer, RingDataBufferType, RingData};
use std::cmp::{min,max};
use opengl_graphics::{ GlGraphics, OpenGL }; 
use piston_window::{Glyphs,G2dTexture};
//use graphics::{Context, Graphics, Transformed, math};
use graphics::*;

use time;
use time::Tm;
use widget::Widget;

const MAX_ENTRIES: usize = 200;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const GREEN_05: [f32; 4] = [0.0, 1.0, 0.0, 0.5];

pub struct HistoRing {
    sliding: bool,
    targetTmMs: Tm,
    size: f64,
    innerrad: f64,
    x: f64,
    y: f64,
    id: i32,
    dat: RingDataBuffer
}

impl HistoRing {
    pub fn new(x: f64, y: f64, 
               size: f64, innerrad: f64, 
               id: i32, 
               dat: state::RingDataBuffer) -> HistoRing {
        println!("new historing size {:?} using data id {:?}",
                 size.clone(), id.clone());
        HistoRing { 
            sliding: false,
            targetTmMs: time::now(),
            size: size,
            innerrad: innerrad,
            x: x, y: y,
            id: id, dat: dat
        }
    }
}

impl<G> Widget<G> for HistoRing
where G: graphics::Graphics
{
    fn draw(
        &mut self,
        //ringbounds: [f64; 4],
        glyphs: Glyphs,
        c: &Context,
        transform: math::Matrix2d,
        g: &mut G,
        //size: f64
        ) {
        let radius = self.size * 0.5;
        let buffer = 2.0;
        let ref mut dat = self.dat;

        //calculate stuff

        let ringbounds=rectangle::centered_square
            (self.x,
             self.y,
             self.size / 2.0);
        //draw stuff
        //rectangle(GREEN,[0.0,-10.0,10.0,10.0], transform, g);
        circle_arc(GREEN_05, 0.5, 0.0, 6.282, ringbounds, transform, g);
        match *dat {
            RingDataBuffer::Ints(ref intsq) => {
                let (sum,mx,avg) = {
                    let sum: i32 = intsq.iter().sum();
                    let mx = intsq.iter().fold(0,|largest, &i| max(i, largest));
                    let avg: f32 = sum as f32 / intsq.len() as f32;
                    //print!("\rs,m,a: {:?} {:?} {:?}", sum, max, avg);
                    (sum,mx,avg)
                };
                let working = (radius - buffer - (radius - self.innerrad + buffer )) as f64;
                let scale = working / mx as f64;
                for (idx, i) in intsq.iter().enumerate() {
                    //println!("draw {:?} {:?}", idx, i.clone());
                    let t = transform.rot_rad(0.031415 * idx as f64);
                    let line = rectangle::rectangle_by_corners(
                        3.0, (1.0 * radius - buffer),
                        -3.0, 
                        ((1.0 * radius - buffer) - (i.clone() as f64 * scale - buffer)).min(1.0 * radius - buffer)
                        );
                    //println!("{:?}", line);
                    rectangle(GREEN_05, line, t, g);
                }
            },
            RingDataBuffer::Text(ref text) => {
                println!("hrtext{:?}", text);
            },
            RingDataBuffer::DatedInts(ref dis) => {
                println!("hrdates{:?}", dis);
            },
            RingDataBuffer::IntVec(ref iv) => {
                println!("hrintvec{:?}", iv);
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
                    RingDataBuffer::Ints(ref mut intq) => 
                    { intq.push_front(i) ;
                        //println!("pushed: {:?}", i)
                      if intq.len() > MAX_ENTRIES
                          { let _ = intq.pop_back();}
                    },
                    _ => {}
                }
            },
            RingData::Text(s) => {},
            RingData::Date(d) => {},
            RingData::IntVec(iv) => {println!("pushed hrintvec")},
        }
    }
}

pub struct GaugesRing {
    sliding: bool,
    targetTmMs: Tm,
    size: f64,
    innerrad: f64,
    x: f64,
    y: f64,
    id: i32,
    dat: RingDataBuffer
}

impl GaugesRing {
    pub fn new(x: f64, y: f64, 
               size: f64, innerrad: f64, 
               id: i32, 
               dat: state::RingDataBuffer) -> GaugesRing {
        println!("new gaugesring size {:?} using data id {:?}",
                 size.clone(), id.clone());
        GaugesRing { 
            sliding: false,
            targetTmMs: time::now(),
            size: size,
            innerrad: innerrad,
            x: x, y: y,
            id: id, dat: dat
        }
    }
}

impl<G> Widget<G> for GaugesRing
where G: graphics::Graphics
{
    fn draw(
        &mut self,
        //ringbounds: [f64; 4],
        glyphs: Glyphs,
        c: &Context,
        transform: math::Matrix2d,
        g: &mut G,
        //size: f64
        ) {
        let radius = self.size * 0.5;
        let buffer = 2.0;
        let ref mut dat = self.dat;

        //calculate stuff
        let ringbounds=rectangle::centered_square
            (self.x,
             self.y,
             self.size / 2.0);

        //draw stuff
        //rectangle(GREEN,[0.0,-10.0,10.0,10.0], transform, g);
        circle_arc(GREEN_05, 0.5, 0.0, 6.282, ringbounds, transform, g);
        match *dat {
            RingDataBuffer::Ints(ref intsq) => {
            },
            RingDataBuffer::Text(ref text) => {
                println!("grtext{:?}", text);
            },
            RingDataBuffer::DatedInts(ref dis) => {
                println!("grdates{:?}", dis);
            },
            RingDataBuffer::IntVec(ref iv) => {
                //println!("griv {:?}", iv);
                let working = (radius - buffer - 
                               (radius - self.innerrad + buffer )) as f64;
                let ringbounds=rectangle::centered_square
                    (self.x,
                     self.y,
                     self.size / 2.0 - self.innerrad / 2.0);
                //let scale = working / mx as f64;
                if let Some(v) = iv.front() {
                    let count = v.len() as f64;
                    let arcsize_max_half = 6.282 / count / 2.0;
                    //println!("arc {:?}", arcsize_max_half) ;
                    for (idx,i) in v.iter().enumerate() {
                        //println!("iterating {:?} {:?}", idx, i) ;
                        let arc_rot = arcsize_max_half * 2.0 * idx as f64;
                        let t = transform.rot_rad(arc_rot);
                        let sz = arcsize_max_half * 0.01 * *i as f64; 
                        circle_arc(GREEN_05, working * 0.5 - buffer, 
                                   - sz, sz,
                                   ringbounds, t, g);
                        }
                }
            }
        }
    }
    fn getid(&mut self) -> i32 { self.id }
    fn push (
        &mut self,
        rdata: state::RingData
        ) {
        match rdata {
            RingData::Int(i) => {}, 
            RingData::Text(s) => {},
            RingData::Date(d) => {},
            RingData::IntVec(iv) => {
                match self.dat {
                    RingDataBuffer::IntVec(ref mut intvecq) => 
                    { intvecq.push_front(iv.clone()) ;
                        //println!("pushed: {:?}", iv);
                      if intvecq.len() > 3
                          { let _ = intvecq.pop_back();}
                    },
                    _ => {}
                }
            },
        }
    }
}

pub struct TextRing {
    sliding: bool,
    targetTmMs: Tm,
    size: f64,
    innerrad: f64,
    x: f64,
    y: f64,
    id: i32,
    dat: RingDataBuffer
}

impl TextRing {
    pub fn new(x: f64, y: f64, 
               size: f64, innerrad: f64, 
               id: i32, 
               dat: state::RingDataBuffer) -> TextRing {
        println!("new gaugesring size {:?} using data id {:?}",
                 size.clone(), id.clone());
        TextRing { 
            sliding: false,
            targetTmMs: time::now(),
            size: size,
            innerrad: innerrad,
            x: x, y: y,
            id: id, dat: dat
        }
    }
}

impl<G> Widget<G> for TextRing
where G: graphics::Graphics
{
    fn draw(
        &mut self,
        //ringbounds: [f64; 4],
        glyphs: Glyphs,
        c: &Context,
        transform: math::Matrix2d,
        g: &mut G,
        //size: f64
        ) {
        let radius = self.size * 0.5;
        let buffer = 2.0;
        let ref mut dat = self.dat;

        //calculate stuff
        let ringbounds=rectangle::centered_square
            (self.x,
             self.y,
             self.size / 2.0);

        //draw stuff
        //rectangle(GREEN,[0.0,-10.0,10.0,10.0], transform, g);
        circle_arc(GREEN_05, 0.5, 0.0, 6.282, ringbounds, transform, g);
        match *dat {
            RingDataBuffer::Ints(ref intsq) => {
                println!("trints{:?}", intsq);
            },
            RingDataBuffer::Text(ref text) => {
                println!("trtext{:?}", text);
                let t = transform.rot_rad(3.0).trans(0.0,radius - buffer);
                let mut txt = text::Text::new_color([0.0,1.0,0.0,0.5], 32);
                txt.draw("x", &mut glyphs,& c.draw_state, t, g);
            },
            RingDataBuffer::DatedInts(ref dis) => {
                println!("trdates{:?}", dis);
            },
            RingDataBuffer::IntVec(ref iv) => {
                println!("trdates{:?}", iv);
            }
        }
    }
    fn getid(&mut self) -> i32 { self.id }
    fn push (
        &mut self,
        rdata: state::RingData
        ) {
        match rdata {
            RingData::Int(i) => {}, 
            RingData::Text(s) => {},
            RingData::Date(d) => {},
            RingData::IntVec(iv) => {
                match self.dat {
                    RingDataBuffer::IntVec(ref mut intvecq) => 
                    { intvecq.push_front(iv.clone()) ;
                        //println!("pushed: {:?}", iv);
                      if intvecq.len() > 3
                          { let _ = intvecq.pop_back();}
                    },
                    _ => {}
                }
            },
        }
    }
}
