// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use core::sync::atomic::{AtomicU64, Ordering};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Instant;

lazy_static! {
    // static ref counter: [u64; 8] = [0; 8];
    static ref counter: [AtomicU64 ; 8] = [0u64; 8].map(|it| AtomicU64 ::new(it));
    static ref timer: Mutex<HashMap<String, Instant>> = Mutex::new(HashMap::new());
    static ref header: Mutex<String> = Mutex::new("".into());
}

// static mut counter: [u64; 8] = [0; 8];

pub fn inc(id: usize) {
    counter[id].fetch_add(1, Ordering::Relaxed);
    counter[id + 4].fetch_add(1, Ordering::Relaxed);
}

pub fn get(id: usize) -> u64 {
    counter[id].fetch_add(0, Ordering::Relaxed)
}

pub fn reset(id: usize) {
    counter[id].store(0, Ordering::Relaxed);
}

fn hum(n: u64) -> String {
    let mut u = "";

    let f = if n > 999999999 {
        u = "g";
        n as f64 / 1000_000_000_f64
    } else if n > 999999 {
        u = "m";
        n as f64 / 1000_000_f64
    } else if n > 999 {
        u = "k";
        n as f64 / 1000_f64
    } else {
        n as f64 / 1_f64
    };

    format!("{:.1}{}", f, u).into()
}

fn th(ms: u128) -> String {
    if ms > 999 {
        format!("{:.1}s", ms as f64 / 1000_f64).into()
    } else {
        format!("{:}", ms).into()
    }
}

pub fn flush() {
    println!(
        "    ** addc[{}, {}] macc[{}, {}]", //
        hum(get(0)),
        hum(get(4)),
        hum(get(2)),
        hum(get(6))
    );

    for i in 0..3 {
        reset(i);
    }
}

pub fn start(name: &str) {
    let mut d = timer.lock().unwrap();

    let t = Instant::now();
    *d.entry(name.into()).or_insert(t) = t;

    flush();
    println!("  __ AP {}", name);
}

pub fn end(name: &str) {
    let mut d = timer.lock().unwrap();

    if !d.contains_key(name) {
        println!("  !!AP bad key {}", name);
        return;
    }

    flush();
    println!("  ^^ AP {}: {} ms", name, th(d[name].elapsed().as_millis()));
}

#[derive(Clone)]
struct Apvar {
    shape: String,
    len: usize,
    dynamic: bool,
}

#[derive(Clone)]
pub struct Ap {
    t: Instant,
    c: u64,
    op: String,
    vars: HashMap<String, Apvar>,
}

pub fn hint(_h: &str) {
    let mut h = header.lock().unwrap();
    h.clear();
    h.push_str(_h);
    h.push_str(" ");
}

pub fn poke() -> Ap {
    let mut ap = Ap { t: Instant::now(), c: get(2), op: "".into(), vars: HashMap::new() };
    ap
}

pub fn peek(msg: &str) {
    let d = timer.lock().unwrap();

    let u = if !d.contains_key("gtimer") { 0 } else { d["gtimer"].elapsed().as_millis() };
    let g0 = d["g0"].elapsed().as_millis();
    let u_mac = get(2) - get(1);
    let mut h = header.lock().unwrap().clone();

    h.push_str(msg);
    println!(
        "{:6} {:25} SEC +{:-4} MAC +{:-6}", //
        th(g0),
        &h,
        th(u),
        hum(u_mac),
    );

    // counter[1].store(get(2), Ordering::Relaxed);
    // *d.entry("gtimer".into()).or_insert(t) = t;
    // h.clear();
}

#[macro_export]
macro_rules! peek_fmt {
    ($ap: expr, $($args:tt),*) => {{
       $ap.peek(&format!($($args),*));
    }};
}

impl Ap {
    pub fn peek(&mut self, msg: &str) {
        let mut d = timer.lock().unwrap();
        let t = Instant::now();

        let _ = *d.entry("g0".into()).or_insert(t);
        let c = self.t.elapsed().as_millis();
        let u = if !d.contains_key("gtimer") { c } else { d["gtimer"].elapsed().as_millis() };

        let g0 = d["g0"].elapsed().as_millis();

        let u_mac = get(2) - get(1);
        let mac = get(2) - self.c;

        let mut h = header.lock().unwrap().clone();
        println!(
            "\n{:>6} {:>5} {}{:4}  {:>6} {}{:6}  {:8}{:48}", //
            th(g0),
            th(c),
            if u > c { "+" } else { "-" },
            th(if u > c { u - c } else { c - u }),
            hum(mac),
            if u > c { "+" } else { "-" },
            hum(if u_mac > mac { u_mac - mac } else { mac - u_mac }),
            &h,
            msg,
        );

        for (k, v) in self.vars.iter() {
            let mut name: String = (if v.dynamic { "*" } else { "" }).into();
            name.push_str(k);
            name.push_str(": ");
            name.push_str(&v.shape);
            println!(
                "{:50} {:>20} {}", //
                "", name, v.len,
            )
        }

        counter[1].store(get(2), Ordering::Relaxed);
        *d.entry("gtimer".into()).or_insert(t) = t;
        // h.clear();

        self.reset();
    }
    pub fn set_const(&mut self, name: &str, shape: &str, len: usize) -> &mut Self {
        let v = Apvar { shape: shape.into(), len, dynamic: false };
        self.vars.entry(name.into()).or_insert(v);
        self
    }
    pub fn set_dynmc(&mut self, name: &str, shape: &str, len: usize) -> &mut Self {
        let v = Apvar { shape: shape.into(), len, dynamic: true };
        self.vars.entry(name.into()).or_insert(v);
        self
    }
    pub fn set_op(&mut self, s: &str) -> &mut Self {
        self.op.clear();
        self.op.push_str(s);
        self
    }
    pub fn reset(&mut self) {
        self.t = Instant::now();
        self.c = get(2);
        self.vars.clear();
        self.op.clear()
    }
}
