extern crate gif;
extern crate image;
extern crate termcolor;
extern crate regex;
use image::{Rgb,RgbImage,DynamicImage,io::Reader as ImageReader};
use std::collections::{HashMap, VecDeque};
use std::{thread,time};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use regex::Regex;

fn gen_palette(img: &RgbImage)->HashMap<[u8;3],(u32,u32)>
{
    let mut cache = HashMap::new();
    for (x,y,p) in img.enumerate_pixels()
    {
        cache.entry(p.0).or_insert((x,y));
    }
    return cache;
}

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
    const DISTTHRESHOLD: f32 = 1.0;

    //? will need to check that neighbors are == col to add to queue. Is a set needed for already flooded then?
    //TODO: needs benchmark but hash map for visited may be faster than repeating sqrt
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
            let temp: (u32, u32) = ((pix.0 as i32 + o.0).clamp(0,img.width() as i32 -1) as u32,
                                        (pix.1 as i32 + o.1).clamp(0,img.height() as i32 -1) as u32);
            let old = *img.get_pixel(temp.0,temp.1);
            if old != col
            // # col_dist(*img.get_pixel(temp.0,temp.1),col)/MAXDIST < DISTTHRESHOLD
            {
                continue;
            }
            //println!("hit");
            &img.put_pixel(temp.0,temp.1,new_col);
            queue.push_back(temp);

        }

        // println!("Queue length {} ", queue.len());
        // thread::sleep(time::Duration::from_secs_f32(0.2));


    }
}

fn main()
{

    let mut line = String::new();
    //?flood on false else replace
    let mut mode:Option<bool> = Option::None;
    while mode == None
    {
        println!("Choose a mode, enter either 'flood' or 'replace'");
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.to_lowercase();
        {
            let re = Regex::new(r"(?i)flood").unwrap();
            if re.is_match(&line)
            {
                mode = Some(false);
                println!("mode chosen: flood");
            } else {
                let re = Regex::new(r"(?i)replace").unwrap();
                if re.is_match(&line)
                {
                    mode = Some(true);
                    println!("mode chosen: replace");
                } else { print!("Invalid Input Text!"); }
            }
        }
    }

    let re = Regex::new(r"(\d+)").unwrap();
    let a: &str = "Adniel Hamed 193 asidogfh 2439807";

    for c in re.captures_iter(a)
    {
        println!("Found {}", &c[0])
    }


    //TODO: print out colors.
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let in_img = ImageReader::open("res/Ferris.png")
        .expect("File not found!")
        .decode()
        .expect("Error decoding!");
    let mut img: RgbImage = in_img.into_rgb8();
    let height: u32 = img.height();
    let width: u32 = img.width();
    let mut cache: HashMap<[u8;3],(u32,u32)> = gen_palette(&img);


    // ? this is a replace mode, maybe take in an input to replace, or do it at seed
    if mode
    {
        //TODO: read in the from color and the to color and check if in map, also add option to print map
        const FROM_COL:[u8;3] = [255,255,255];
        while cache.contains_key(&FROM_COL)
        {
            flood(&mut img, Rgb::from(FROM_COL), Rgb::from([200, 255, 255]), *cache.get(&FROM_COL).unwrap());
            cache.clear();
            cache = gen_palette(&img);
        }
    }
    else
    {
        //? this is the flood mode
        //todo: read in a seed, use the get pixel as the from color for the flood and read in the new color

    }


    img.save("Output.png").expect("Failed to write image");

}
