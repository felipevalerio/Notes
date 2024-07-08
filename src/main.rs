use std::{fs, u8};
use std::f32::consts::PI;
use std::io::Write;


const DURATION: u32 = 1;
const SAMPLE_RATE: u32 = 44100;
// const FREQUENCY: u32 = 493;


fn create_sin_wave(angle: f32, nsamples: u32, frequency: i32, mut buffer: [u8; 4]) {


    let mut data_file = fs::OpenOptions::new()
        .append(true)
        .open("out.bin")
        .expect("erro na abertura do arquivo");

    for i in 0..nsamples {

        let sample: f32 = (angle * frequency as f32 * i as f32).sin();
        let sample_bits: u32 = sample.to_bits();
        buffer.copy_from_slice(&sample_bits.to_le_bytes());
        data_file.write(&buffer).expect("erro");
    }

}


fn main() {

    let nsamples: u32 = DURATION * SAMPLE_RATE;
    let result: f32 = PI * 2.0;
    let angle: f32 = result / nsamples as f32;

    let buffer = [0u8; 4];

    let frequencies: [i32; 8] = [261, 293, 329, 349, 392, 440, 493, 523]; // cada item é a frequência de uma nota na escala de Dó

    for frequency in frequencies {
        create_sin_wave(angle, nsamples, frequency, buffer);
    }
    
}