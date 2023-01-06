extern crate gif;
extern crate image;
extern crate regex;
use image::{Rgb,RgbImage,DynamicImage,io::Reader as ImageReader};
use std::collections::{HashMap, VecDeque};
use std::{thread,time};
use std::fmt::format;
use std::fs::{read_dir, File};
use regex::Regex;
use gif::{Frame,Encoder,Repeat};

fn gen_palette(img: &RgbImage)->HashMap<[u8;3],(u32,u32)>
{
    let mut cache = HashMap::new();
    for (x,y,p) in img.enumerate_pixels()
    {
        cache.entry(p.0).or_insert((x,y));
    }
    return cache;
}

fn flood( img: &mut RgbImage, col:Rgb<u8>, new_col:Rgb<u8>, seed:(u32,u32))
{

    //? will need to check that neighbors are == col to add to queue. Is a set needed for already flooded then?
    //TODO: needs benchmark but hash map for visited may be faster than repeating sqrt
    let mut queue = VecDeque::new();
    queue.push_back(seed);

    const OFFSETS:[(i32,i32); 8] = [(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1),(0,-1),(1,-1)];
    img.put_pixel(seed.0,seed.1,new_col);

    let mut ind:i32 = 0;

    while !queue.is_empty()
    {
        let pix = queue.pop_front().unwrap();



        for o in OFFSETS
        {
            let temp: (u32, u32) = ((pix.0 as i32 + o.0).clamp(0,img.width() as i32 -1) as u32,
                                        (pix.1 as i32 + o.1).clamp(0,img.height() as i32 -1) as u32);
            let old = *img.get_pixel(temp.0,temp.1);
            if old != col
            {
                continue;
            }
            let _ = &img.put_pixel(temp.0,temp.1,new_col);
            ind += 1;

            const STEP:i32 = 100;

            if ind%STEP == 0
            {
                img.save(format!("temp/FrameOutput_{}.jpg",format!("{:03}",ind/STEP))).expect("Error Saving");
            }
            queue.push_back(temp);

        }


    }
}

fn input_pos(x:u32, y:u32) -> (u32,u32)
{
    let mut line:String = String::new();
    let mut position:Option<(u32,u32)> = Option::None;
    while position == None
    {
        println!("enter a position in the format x,y ");
        std::io::stdin().read_line(&mut line).unwrap();
        let re = Regex::new(r"(\d+)").unwrap();
        let mut cap = re.captures_iter(&line);


        if re.captures_len() >= 1
        {

            match cap.nth(0).unwrap().get(0).unwrap().as_str().parse::<i32>()
            {
                Ok(i) => position = Some((i.clamp(0,x as i32) as u32,i as u32)),
                Err(e) => position = Option::None
            }

            if position != None
            {
                match cap.nth(0).unwrap().get(1).unwrap().as_str().parse::<i32>()
                {
                    Ok(j) => position = Some((position.unwrap().0 as u32, j.clamp(0,y as i32) as u32)),
                    Err(e) => position = Option::None
                }
            }
        }
        else { println!("not enough coordinates!"); }


    }
    return position.unwrap();
}

fn input_col() -> [u8;3]
{
    let mut line:String = String::new();
    let mut col:Option<[u8;3]> = None;
    while col == None
    {
        println!("enter a position in the format R,G,B such as 255,255,255 ");
        std::io::stdin().read_line(&mut line).unwrap();
        let re = Regex::new(r"(\d+)").unwrap();
        let mut cap = re.captures_iter(&line);


        if re.captures_len() >= 2
        {

            match cap.nth(0).unwrap().get(0).unwrap().as_str().parse::<u8>()
            {
                Ok(i) => col = Some([i,i,i]),
                Err(e) => col = None
            }

            if col != None
            {
                match cap.nth(0).unwrap().get(1).unwrap().as_str().parse::<u8>()
                {
                    Ok(j) => col = Some([col.unwrap()[0],j,j]),
                    Err(e) => col = None
                }
                if col != None
                {
                    match cap.nth(0).unwrap().get(1).unwrap().as_str().parse::<u8>()
                    {
                        Ok(k) => col = Some([col.unwrap()[0],col.unwrap()[1],k]),
                        Err(e) => col = None
                    }
                }
            }
        }
        else { println!("not enough coordinates!"); }


    }
    return col.unwrap();
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



    let in_img = ImageReader::open("res/ferris.png")
        .expect("File not found!")
        .decode()
        .expect("Error decoding!");
    let mut img: RgbImage = in_img.into_rgb8();
    let height: u32 = img.height();
    let width: u32 = img.width();


    // ? this is a replace mode, maybe take in an input to replace, or do it at seed
    if mode.unwrap()
    {
        let mut cache: HashMap<[u8;3],(u32,u32)> = gen_palette(&img);
        println!("Enter the color to replace");
        let from_col = input_col();
        println!("Enter the color to replace it with");
        let to_col = input_col();
        while cache.contains_key(&from_col)
        {
            flood(&mut img, Rgb::from(from_col), Rgb::from(to_col), *cache.get(&from_col).unwrap());
            cache.clear();
            cache = gen_palette(&img);
        }
    }
    else
    {
        let seed = input_pos(width,height);
        let col = *img.get_pixel(seed.0,seed.1);
        println!("Enter the color to flood");
        let to_col = input_col();
        flood(&mut img, col,Rgb::from(to_col),seed);

    }


    img.save("Output.png").expect("Failed to write image");

    //? make the gif

    println!("Files: {}", read_dir("./temp").unwrap().count());

    let mut image = File::create("out.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut image, width as u16,height as u16,&[]).unwrap();

    for file in read_dir("./temp").unwrap()
    {
        println!("file {}", file.as_ref().unwrap().file_name().into_string().unwrap());
        let mut img: RgbImage = ImageReader::open(format!("temp/{}",&file.unwrap().file_name().into_string().unwrap())).unwrap().decode().unwrap().into_rgb8();
        let mut pixels: Vec<u8> = vec![];
        for &p in img.pixels()
        {
            pixels.push(p.0[0]);
            pixels.push(p.0[1]);
            pixels.push(p.0[2]);
        }
        println!("Width: {}, Height: {}, mult: {}, len: {}", width,height,(width*height *3),pixels.len());
        let frame = gif::Frame::from_rgb(width as u16,height as u16,&mut pixels);
        encoder.write_frame(&frame).unwrap();
    }





}
