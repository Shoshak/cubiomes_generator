#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(integer_sign_cast)]

use std::{env, process};

include!("../out_bindings/bindings.rs");

const STRUCT_TYPE: i32 = StructureType_Treasure.cast_signed();
const MC_VERSION: i32 = MCVersion_MC_1_19.cast_signed();

#[derive(Debug)]
struct Position {
    x: i32,
    z: i32,
}

fn get_generator(seed: u64) -> Generator {
    let mut g = Generator {
        mc: MC_VERSION,
        dim: Dimension_DIM_OVERWORLD,
        flags: 0,
        seed,
        sha: 0,
        ..Default::default()
    };
    unsafe {
        setupGenerator(&mut g, MC_VERSION, 0);
        applySeed(&mut g, Dimension_DIM_OVERWORLD, seed);
    }
    g
}

fn gen_attempt(coords: &Position, seed: u64) -> Option<Position> {
    let mut p = Pos { x: 0, z: 0 };
    unsafe {
        if getStructurePos(STRUCT_TYPE, MC_VERSION, seed, coords.x, coords.z, &mut p) == 1 {
            Some(Position { x: p.x, z: p.z })
        } else {
            None
        }
    }
}

fn is_viable_location(generator: &mut Generator, coords: &Position) -> bool {
    (unsafe { isViableStructurePos(STRUCT_TYPE, generator, coords.x, coords.z, 0) } == 1)
}

const fn hyp_distance(p1: &Position, p2: &Position) -> i32 {
    (p2.x - p1.x).pow(2) + (p2.z - p1.z).pow(2)
}

fn gen(generator: &mut Generator, seed: u64, starting_position: &Position) -> Option<Position> {
    for offset in (0..100_000).step_by(16) {
        let x_neg_z_iter = (starting_position.x - offset..starting_position.x + offset)
            .step_by(16)
            .map(|x| Position {
                x,
                z: starting_position.z - offset,
            });
        let x_pos_z_iter = (starting_position.x - offset..starting_position.x + offset)
            .step_by(16)
            .map(|x| Position {
                x,
                z: starting_position.z + offset,
            });
        let z_neg_x_iter = (starting_position.z - offset..starting_position.z + offset)
            .step_by(16)
            .map(|z| Position {
                x: starting_position.x - offset,
                z,
            });
        let z_pos_x_iter = (starting_position.z - offset..starting_position.z + offset)
            .step_by(16)
            .map(|z| Position {
                x: starting_position.x + offset,
                z,
            });
        let combined = x_neg_z_iter
            .chain(x_pos_z_iter)
            .chain(z_neg_x_iter)
            .chain(z_pos_x_iter);
        if let Some(found_pos) = combined
            .filter_map(|p| {
                let gen_p = gen_attempt(
                    &Position {
                        x: p.x / 16,
                        z: p.z / 16,
                    },
                    seed,
                )?;
                if is_viable_location(generator, &gen_p) {
                    Some(gen_p)
                } else {
                    None
                }
            })
            .min_by(|p1, p2| {
                hyp_distance(starting_position, p1).cmp(&hyp_distance(starting_position, p2))
            })
        {
            return Some(found_pos);
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let seed = args
        .get(1)
        .unwrap_or_else(|| {
            eprintln!("Argument for seed is not supplied");
            process::exit(1);
        })
        .parse::<i64>()
        .unwrap_or_else(|err| {
            eprintln!("Could not parse seed with error: {err}");
            process::exit(1);
        })
        .cast_unsigned();
    let starting_x = args
        .get(2)
        .unwrap_or_else(|| {
            eprintln!("Argument for x is not supplied");
            process::exit(1);
        })
        .parse::<i32>()
        .unwrap_or_else(|err| {
            eprintln!("Could not parse x coordinate with error: {err}");
            process::exit(1);
        });
    let starting_z = args
        .get(3)
        .unwrap_or_else(|| {
            eprintln!("Argument for z is not supplied");
            process::exit(1);
        })
        .parse::<i32>()
        .unwrap_or_else(|err| {
            eprintln!("Could not parse z coordinate with error: {err}");
            process::exit(1);
        });
    let mut generator = get_generator(seed);
    let starting_position = &Position {
        x: starting_x,
        z: starting_z,
    };
    let pos = gen(&mut generator, seed, starting_position);
    if let Some(p) = pos {
        println!("{} {}", p.x, p.z);
    } else {
        eprintln!("not found");
    }
}
