use crate::day01::{day01_part1, day01_part2};
mod day01;
fn day01() {
    assert_eq!(72_240, day01_part1());
    assert_eq!(210_957, day01_part2());
}

use crate::day02::{day02_part1, day02_part2};
mod day02;
fn day02() {
    assert_eq!(12_276, day02_part1());
    assert_eq!(9_975, day02_part2());
}

use crate::day03::{day03_part1, day03_part2};
mod day03;
fn day03() {
    assert_eq!(7_763, day03_part1());
    assert_eq!(2_569, day03_part2());
}

use crate::day04::{day04_part1, day04_part2};
mod day04;
fn day04() {
    assert_eq!(507, day04_part1());
    assert_eq!(897, day04_part2());
}

use crate::day05::{day05_part1, day05_part2};
mod day05;
fn day05() {
    assert_eq!("RTGWZTHLD", day05_part1());
    assert_eq!("STHGRZZFR", day05_part2());
}

use crate::day06::{day06_part1, day06_part2};
mod day06;
fn day06() {
    assert_eq!(1_876, day06_part1());
    assert_eq!(2_202, day06_part2());
}

use crate::day07::{day07_part1, day07_part2};
mod day07;
fn day07() {
    assert_eq!(1_543_140, day07_part1());
    assert_eq!(1_117_448, day07_part2());
}

use crate::day08::{day08_part1, day08_part2};
mod day08;
fn day08() {
    assert_eq!(1_832, day08_part1());
    assert_eq!(157_320, day08_part2());
}

use crate::day09::{day09_part1, day09_part2};
mod day09;
fn day09() {
    assert_eq!(6_311, day09_part1());
    assert_eq!(2_482, day09_part2());
}

use crate::day10::{day10_part1, day10_part2, PART2_RESULT_IMAGE};
mod day10;
fn day10() {
    assert_eq!(14_780, day10_part1());
    assert_eq!(PART2_RESULT_IMAGE, day10_part2());
}

use crate::day11::{day11_part1, day11_part2};
mod day11;
fn day11() {
    assert_eq!(54_054, day11_part1());
    assert_eq!(14_314_925_001, day11_part2());
}

use crate::day12::{day12_part1, day12_part2};
mod day12;
fn day12() {
    assert_eq!(380, day12_part1());
    assert_eq!(375, day12_part2());
}

use crate::day13::{day13_part1, day13_part2};
mod day13;
fn day13() {
    assert_eq!(6_369, day13_part1());
    assert_eq!(25_800, day13_part2());
}

use crate::day14::{day14_part1, day14_part2};
mod day14;
fn day14() {
    assert_eq!(913, day14_part1());
    assert_eq!(30_762, day14_part2());
}

use crate::day15::{day15_part1, day15_part2};
mod day15;
fn day15() {
    assert_eq!(5_256_611, day15_part1());
    assert_eq!(13_337_919_186_981, day15_part2());
}

use crate::day16::{day16_part1, day16_part2};
mod day16;
fn day16() {
    assert_eq!(1_488, day16_part1());
    assert_eq!(2_111, day16_part2());
}

use crate::day17::{day17_part1, day17_part2};
mod day17;
fn day17() {
    assert_eq!(3_071, day17_part1());
    assert_eq!(1_523_615_160_362, day17_part2());
}

use crate::day18::{day18_part1, day18_part2};
mod day18;
fn day18() {
    assert_eq!(3_454, day18_part1());
    assert_eq!(2_014, day18_part2());
}

use crate::day19::{day19_part1, day19_part2};
mod day19;
fn day19() {
    assert_eq!(1_550, day19_part1());
    assert_eq!(18_630, day19_part2());
}

use crate::day20::{day20_part1, day20_part2};
mod day20;
fn day20() {
    assert_eq!(13_289, day20_part1());
    assert_eq!(2_865_721_299_243, day20_part2());
}

use crate::day21::{day21_part1, day21_part2};
mod day21;
fn day21() {
    assert_eq!(110_181_395_003_396, day21_part1());
    assert_eq!(3_721_298_272_959, day21_part2());
}

use crate::day22::{day22_part1, day22_part2};
mod day22;
fn day22() {
    assert_eq!(31_568, day22_part1());
    assert_eq!(36_540, day22_part2());
}

use crate::day23::{day23_part1, day23_part2};
mod day23;
fn day23() {
    assert_eq!(3_874, day23_part1());
    assert_eq!(948, day23_part2());
}

use crate::day24::{day24_part1, day24_part2};
mod day24;
fn day24() {
    assert_eq!(308, day24_part1());
    assert_eq!(908, day24_part2());
}

use crate::day25::day25_part1;
mod day25;
fn day25() {
    assert_eq!("2-212-2---=00-1--102", day25_part1());
}

fn main() {
    day01();
    day02();
    day03();
    day04();
    day05();
    day06();
    day07();
    day08();
    day09();
    day10();
    day11();
    day12();
    day13();
    day14();
    day15();
    day16();
    day17();
    day18();
    day19();
    day20();
    day21();
    day22();
    day23();
    day24();
    day25();
}
