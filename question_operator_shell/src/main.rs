use std::fs::File;
use std::io;
use image::codecs::png::PngEncoder;
use image::{ExtendedColorType,ImageEncoder,ImageError};
use rand::Rng;
use num::Complex;
fn test_question_mark_operator()->Result<String,io::Error>{
  File::open("nonexistentfile.txt")?;
  // match res{
  //   Ok(file)=>{
  //     println!("file opened");
  //   }
  //   Err(err)=>{return Err(err);}
  // }
  Ok("open file ok".to_string())
}
fn write_png(filename:&str,pixels:&[u8],dimensions:(usize,usize))->Result<(),ImageError>{
  let output=File::create(filename)?;
  let encoder=PngEncoder::new(output);
  encoder.write_image(&pixels, dimensions.0 as u32,dimensions.1 as u32, ExtendedColorType::L8);
  Ok(())
}
fn main() {
    // let res=test_question_mark_operator();
    // match res{
    //   Ok(s)=>println!("good"),
    //   Err(e)=>println!("not opened withe error:{:?}",e),
    // }
    let png_width=640;
    let png_height=480;
    let file_name="gray.png";
    let mut img_buf=vec![0;png_width*png_height];
    for idx in 0..png_width*png_height{
      img_buf[idx]=rand::thread_rng().gen_range(0..=255);
    }
    let res=write_png(file_name, &img_buf, (png_width,png_height));
    match res{
      Ok(())=>{
        println!("completed");
      }
      Err(err)=>{

        println!("error:{:?}",err);
      }
    }
    
}
