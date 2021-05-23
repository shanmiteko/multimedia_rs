// #![windows_subsystem = "windows"]

use std::{
    fs::File,
    io::{Cursor, Read},
    thread,
    time::Duration,
};

use byteorder::{LittleEndian, ReadBytesExt};
use sfml::audio::{SoundStatus, SoundStream, SoundStreamPlayer};
use thread::JoinHandle;

const BUF_SIZE: usize = 2048;

struct SquareWave {
    buf: [i16; BUF_SIZE],
    freq: u32,
    max: i16,
    min: i16,
    has_more: i32,
}

impl SquareWave {
    fn new(max: i16, freq: u32, has_more: i32) -> SquareWave {
        SquareWave {
            buf: [0; BUF_SIZE],
            freq,
            max,
            min: 0,
            has_more,
        }
    }
}

impl SoundStream for SquareWave {
    fn get_data(&mut self) -> (&mut [i16], bool) {
        let max = self.max;
        let min = self.min;
        let freq = self.freq;

        let mut is_max = false;

        self.buf.iter_mut().enumerate().for_each(|(n, b)| {
            if n % (BUF_SIZE / freq as usize) == 0 {
                is_max = !is_max;
            }
            *b = if is_max { max } else { min }
        });

        self.has_more -= 1;

        (&mut self.buf[..], self.has_more > 0)
    }

    fn seek(&mut self, offset: sfml::system::Time) {
        println!("SquareWave seek {:?}", offset);
    }

    fn channel_count(&self) -> u32 {
        1
    }

    fn sample_rate(&self) -> u32 {
        44_100
    }
}

/// 16bit PCM 数据
struct PcmData {
    /// 一帧缓冲
    buf: [i16; BUF_SIZE],
    file_handle: File,
}

impl PcmData {
    fn from_file(file_name: &str) -> PcmData {
        PcmData {
            buf: [0; BUF_SIZE],
            file_handle: File::open(file_name).unwrap(),
        }
    }
}

impl SoundStream for PcmData {
    fn get_data(&mut self) -> (&mut [i16], bool) {
        // 临时储存一帧16位PCM
        let mut buf = [0; 2 * BUF_SIZE];

        let mut read_size = 0;

        // 按u8格式读取到buf中
        if let Ok(size) = self.file_handle.read(&mut buf) {
            read_size = size;
        };

        if read_size > 0 {
            // 有Seek实现
            let mut cursor_buf = Cursor::new(buf);

            self.buf.iter_mut().for_each(|pcm| {
                *pcm = cursor_buf.read_i16::<LittleEndian>().unwrap();
            });
        }

        (&mut self.buf[..], read_size > 0)
    }

    fn seek(&mut self, offset: sfml::system::Time) {
        println!("Pcm seek {:?}", offset);
    }

    fn channel_count(&self) -> u32 {
        2
    }

    fn sample_rate(&self) -> u32 {
        44_100
    }
}

fn play<T: SoundStream>(mut wave: T) {
    let mut player = SoundStreamPlayer::new(&mut wave);

    player.play();

    // 循环检测是否正在播放 防止主程序退出
    while player.status() == SoundStatus::Playing {
        // 防止空转次数过多
        thread::sleep(Duration::from_millis(100));
    }
}

fn play_spawn<T: 'static + SoundStream + Send>(wave: T) -> JoinHandle<()> {
    thread::spawn(move || play(wave))
}

fn main() {
    let mut handles = Vec::<JoinHandle<()>>::new();

    let i16_max = (2_i32.pow(15) - 1) as i16;

    play(SquareWave::new(i16_max, 32, 1));

    handles.push(play_spawn(PcmData::from_file(".\\assets\\live.pcm")));

    play(SquareWave::new(i16_max, 32, 1));

    for handle in handles {
        handle.join().unwrap()
    }
}
