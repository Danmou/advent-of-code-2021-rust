use std::fs::File;
use std::io::{BufRead, BufReader};
use bitvec::prelude::*;
use bitvec::slice::Iter;
use hex::FromHex;

fn read_input() -> BitVec<Msb0, u8> {
    let file = File::open("inputs/16.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let line = br.lines().next().unwrap().unwrap();
    Vec::from_hex(line).unwrap().iter().collect()
}

fn bits_to_int(bits: &BitSlice<Msb0, u8>) -> u64 {
    let mut v = 0;
    v.view_bits_mut::<Msb0>()[64-bits.len()..].clone_from_bitslice(bits);
    v
}

fn take_bits(iter: &mut Iter<Msb0, u8>, bit_count: usize) -> Option<BitVec<Msb0, u8>> {
    let chunk: BitVec<Msb0, u8> = iter.take(bit_count).collect();
    if chunk.len() < bit_count {
        return None;
    }
    Some(chunk)
}

fn parse_int(iter: &mut Iter<Msb0, u8>, bit_count: usize) -> Option<u64> {
    let chunk = take_bits(iter, bit_count)?;
    Some(bits_to_int(chunk.as_bitslice()))
}

fn parse_literal(iter: &mut Iter<Msb0, u8>) -> Option<u64> {
    let mut bits: BitVec<Msb0, u8> = BitVec::new();
    loop {
        let chunk = take_bits(iter, 5)?;
        bits.extend(chunk[1..].iter());
        if chunk[0] == false {
            break;
        }
    }
    Some(bits_to_int(bits.as_bitslice()))
}

fn parse_operands(iter: &mut Iter<Msb0, u8>) -> Option<Vec<u64>> {
    let length_type = parse_int(iter, 1)?;
    if length_type == 0 {
        let num_bits = parse_int(iter, 15)?;
        let bits = take_bits(iter, num_bits as usize)?;
        Some(parse(&bits))
    } else {
        let num_packets = parse_int(iter, 11)?;
        let mut vals = Vec::new();
        for _ in 0..num_packets {
            vals.push(parse_packet(iter)?);
        }
        Some(vals)
    }
}

fn parse_packet(iter: &mut Iter<Msb0, u8>) -> Option<u64> {
    let version = parse_int(iter, 3)?;
    let id = parse_int(iter, 3)?;
    return if id == 4 {
        let val = parse_literal(iter)?;
        println!("Literal: {}", val);
        Some(val)
    } else {
        println!("Operator type {} begin", id);
        let operands = parse_operands(iter)?;
        println!("Operator type {} end", id);
        if operands.is_empty() {
            panic!();
        }
        Some(match id {
            0 => operands.iter().sum(),
            1 => operands.iter().product(),
            2 => *operands.iter().min().unwrap(),
            3 => *operands.iter().max().unwrap(),
            5 => if operands[0] > operands[1] {1} else {0},
            6 => if operands[0] < operands[1] {1} else {0},
            7 => if operands[0] == operands[1] {1} else {0},
            _ => panic!(),
        })
    }
}

fn parse(bits: &BitVec<Msb0, u8>) -> Vec<u64> {
    let mut iter = bits.iter();
    let mut results = Vec::new();
    while iter.len() > 0 {
        let res = parse_packet(&mut iter);
        if res.is_none() {
            if iter.len() != 0 {
                panic!("{} unparsed bits left", iter.len());
            }
            break;
        }
        results.push(res.unwrap());
    }
    results
}

fn main() {
    let bits = read_input();
    println!("Result: {:?}", parse(&bits));
}
