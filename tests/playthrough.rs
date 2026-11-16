//! End-to-end regression tests built from the walkthrough playtest scripts.
//!
//! Each fixture is a cumulative transcript: `ch2.txt` replays chapters 1-2,
//! `ch7.txt` replays the whole game.  They were validated by hand against the
//! walkthrough's score milestones, so they also guard the scoring rules.

use hexuma::game::Game;

fn play(script: &str) -> Game {
    let mut game = Game::new();
    for line in script.lines() {
        let line = line.trim();
        if line.is_empty() || line == "quit" || line == "exit" {
            continue;
        }
        game.command(line);
    }
    game
}

fn check(script: &str, score: i32, shards: i32) -> Game {
    let game = play(script);
    assert_eq!(game.score, score, "unexpected score");
    assert_eq!(game.shards(), shards, "unexpected gem splinters");
    game
}

#[test]
fn chapter1_reaches_usa() {
    let game = check(include_str!("scripts/ch1.txt"), 275, 1);
    assert_eq!(game.room, 120);
}

#[test]
fn chapter2_usa_museum() {
    check(include_str!("scripts/ch2.txt"), 500, 2);
}

#[test]
fn chapter3_ice_world() {
    check(include_str!("scripts/ch3.txt"), 615, 3);
}

#[test]
fn chapter4_shipwreck() {
    check(include_str!("scripts/ch4.txt"), 895, 4);
}

#[test]
fn chapter5_prehistory() {
    check(include_str!("scripts/ch5.txt"), 1020, 5);
}

#[test]
fn chapter6_mine_and_forge() {
    check(include_str!("scripts/ch6.txt"), 1245, 5);
}

#[test]
fn chapter7_ends_aboard_the_spaceship() {
    let game = check(include_str!("scripts/ch7.txt"), 1595, 5);
    assert_eq!(game.room, 238);
    assert!(game.inv.contains(&"eye"), "the Eye of Kal should be in hand");
}
