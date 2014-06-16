extern crate std;

use std::cmp;
use std::iter::AdditiveIterator;


fn score_sum(frame_number: uint, pins: &[uint]) -> uint {
    match pins.len() {
        0 => 0,
        _ => frame_score(pins) + score_sum(frame_number + 1,
                                           next_frame(pins, frame_number))
    }
}


fn frame_score(pins: &[uint]) -> uint {
    if is_strike(pins) || is_spare(pins) {
        fold_score(pins, 3)
    } else {
        fold_score(pins, 2)
    }
}


fn next_frame<'a>(pins: &'a [uint], frame_number: uint) -> &'a [uint] {
    if frame_number == 10 || pins.len() < 1 {
        &'static []
    } else if is_strike(pins) {
        pins.tail()
    } else {
        pins.tailn(2)
    }
}


fn is_strike(pins: &[uint]) -> bool {
    fold_score(pins, 1) == 10
}


fn is_spare(pins: &[uint]) -> bool {
    fold_score(pins, 2) == 10
}


fn fold_score(pins: &[uint], max_throws: uint) -> uint {
    let length = cmp::min(max_throws, pins.len());
    pins.slice(0, length).iter().map(|&x| x).sum()
}


fn total(pins: &[uint]) -> uint {
    score_sum(1, pins)
}


static ZEROS: &'static [uint] = &[
    0, 0, 0, 0,
    0, 0, 0, 0,
    0, 0, 0, 0,
    0, 0, 0, 0,
    0, 0, 0, 0];

static ONES: &'static [uint] = &[
    1, 1, 1, 1,
    1, 1, 1, 1,
    1, 1, 1, 1,
    1, 1, 1, 1,
    1, 1, 1, 1];

static SPARE: &'static [uint] = &[
    5, 5, 3, 0,
    0, 0, 0, 0,
    0, 0, 0, 0,
    0, 0, 0, 0,
    0, 0, 0, 0];

static STRIKE: &'static [uint] = &[
    10,   3, 4,
    0, 0, 0, 0,
    0, 0, 0, 0,
    0, 0, 0, 0,
    0, 0, 0, 0];

static PERFECT: &'static [uint] = &[
    10,   10,
    10,   10,
    10,   10,
    10,   10,
    10,   10,
    10,   10];


fn main() {
    assert_eq!(total(ZEROS), 0);
    assert_eq!(total(ONES), 20);
    assert_eq!(total(SPARE), 16);
    assert_eq!(total(STRIKE), 24);
    assert_eq!(total(PERFECT), 300);
}

