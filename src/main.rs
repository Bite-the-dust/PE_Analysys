use std::fs::File;
use std::io::SeekFrom;
use std::io::Read;
use std::io;

fn main() -> io::Result<()>{
    let filename = "C:\\Users\\tiwasaki\\Desktop\\PE_Analysys\\test.exe";
    //let mut file = File::open(filename).expect("file not found");
    
    let mut bytes = vec![];
    let mut f = File::open(filename)?;
    let _ = f.read_to_end(&mut bytes)?;
    let ptr = 128+0x18+0x12+0x60; 
    println!("{:?}", &bytes[ptr..ptr+0x10]);
    println!("{:?}", (&bytes[ptr+0x10..ptr+0x20]));
    println!("{:?}", (&bytes[ptr+0x20..ptr+0x30]));
    let ptr2:Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 96];
    let inte = u64::from_be_bytes(ptr2[0..8].try_into().unwrap());
    //14c0
    println!("{:?}", String::from_utf8_lossy(&bytes[0x14c0..0x14c0+0x100]));    println!("{:?}", String::from_utf8_lossy(&bytes[5345..5345+0x100])); 
    println!("{}", inte);

    Ok(())
}
