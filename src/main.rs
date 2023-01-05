extern crate gif;
extern crate image;
use image::{Rgb,RgbImage,DynamicImage,io::Reader as ImageReader};
use std::collections::{HashMap, VecDeque};
use std::{thread,time};


fn col_dist(col:Rgb<u8>, new_col:Rgb<u8>) ->f32
{
    return ((col[0] as f32 - new_col[0] as f32).powf(2.0) +
                            (col[1] as f32 - new_col[1] as f32).powf(2.0) +
                            (col[2] as f32 - new_col[2] as f32).powf(2.0)).sqrt();

}

fn flood( img: &mut RgbImage, col:Rgb<u8>, new_col:Rgb<u8>, seed:(u32,u32))
{
    //$ Change threshold for tightness
    const MAXDIST: f32 = 442.0;
    const DISTTHRESHOLD: f32 = 0.6;
    
    //? will need to check that neighbors are == col to add to queue. Is a set needed for already flooded then?
    let mut queue = VecDeque::new();
    queue.push_back(seed);

    const OFFSETS:[(i32,i32); 8] = [(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1),(0,-1),(1,-1)];
    img.put_pixel(seed.0,seed.1,new_col);

    while !queue.is_empty()
    {
        //! needs to clamp summed values to width height of img
        let pix = queue.pop_front().unwrap();



        for o in OFFSETS
        {
            //clamp to in image
            let mut temp: (u32, u32) = ((pix.0 as i32 + o.0).clamp(0,img.width() as i32 -1) as u32,
                                        (pix.1 as i32 + o.1).clamp(0,img.height() as i32 -1) as u32);
            //TODO: panicking because it hits out of bounds
            if col_dist(*img.get_pixel(temp.0,temp.1),col)/MAXDIST > DISTTHRESHOLD
            {
                continue;
            }
            else
            {
                queue.push_back(temp);
                img.put_pixel(temp.0,temp.1,new_col);
            }
        }
        //
        // println!("Queue length {} ", queue.len());
        // thread::sleep(time::Duration::from_secs_f32(0.2));

        if queue.len() > 200
        {
            break;
        }

    }
}

fn main()
{
    println!("Hello, world!");
    let in_img = ImageReader::open("res/ferris.png")
        .expect("File not found!")
        .decode()
        .expect("Error decoding!");
    let mut img: RgbImage = in_img.into_rgb8();
    let height: u32 = img.height();
    let width: u32 = img.width();
    let mut cache = HashMap::new();

    for (x,y,p) in img.enumerate_pixels()
    {
        cache.entry(p.0).or_insert((x,y));
    }

    for (k,v) in cache
    {
        //these are seeds for the flood fill
        //println!("Pixel is {},{},{} at {},{}", k[0],k[1],k[2],v.0,v.1);
    }
    flood(&mut img,Rgb::from([255,255,255]),Rgb::from([20,20,20]),(0,0));

    img.save("Output.png").expect("Failed to write image");

}
