#![feature(test)]

use std::time::{Instant};
use ringbuf::{RingBuffer, Producer, Consumer};

pub struct RbBencher;

impl RbBencher {
    pub fn run(rb_capacity: usize, total_transfer: usize, buf_size: usize) {
        assert!(rb_capacity >= buf_size);

        println!("rb_capacity = {} bytes", rb_capacity);
        println!("total_transfer = {} bytes", total_transfer);
        println!("buf_size = {} bytes", buf_size);
    
        println!("Start running the benchmark...");
        let rb = RingBuffer::<u8>::new(rb_capacity);
        let (mut producer, mut consumer) = rb.split();

        let mut count = 0;
        let input_buf = std::hint::black_box(vec![0_u8; buf_size]);
        let mut output_buf = vec![0_u8; buf_size];
        while count < total_transfer {
            let nbytes = producer.push_slice(&input_buf[..buf_size]);
            debug_assert!(nbytes == buf_size);

            let nbytes = consumer.pop_slice(&mut output_buf[..buf_size]);
            debug_assert!(nbytes == buf_size);

            count += buf_size;
        }
        std::hint::black_box(output_buf);
    }
}

fn main() {
    let rb_capacity : usize = 1024 * 1024;
    let total_transfer : usize = 16 * 1024 * 1024 * 1024; 
    let buf_size : usize = 4 * 1024;
    
    let from = Instant::now();

    RbBencher::run(rb_capacity, total_transfer, buf_size);

    let elapsed = from.elapsed();
    let secs = elapsed.as_secs_f64();
    let throughput = (total_transfer as f64) / 1000.0 / 1000.0 / secs;
    println!("throughput = {} MB/s", throughput);
}
