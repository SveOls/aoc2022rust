use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::*;

#[derive(Clone, Debug)]
enum Monkey {
    Val(i64),
    Operation(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>, char),
    Null([u8; 4], [u8; 4], char),
}

impl Monkey {
    // fn collapse(&self) -> Option<i64> {
    //     match self {
    //         Monkey::Val(a) => Some(*a),
    //         Monkey::Operation(l, r, '+') => l.borrow().val() + r.borrow().val(),
    //         Monkey::Operation(l, r, '*') => l.borrow().val() * r.borrow().val(),
    //         Monkey::Operation(l, r, '/') => l.borrow().val() / r.borrow().val(),
    //         Monkey::Operation(l, r, '-') => l.borrow().val() - r.borrow().val(),
    //         _ => panic!(),
    //     }
    // }
    fn eq(&self) -> i64 {
        if let Monkey::Operation(l, r, _) = self {
            l.borrow().val() - r.borrow().val()
        } else {
            panic!()
        }
    }
    fn val(&self) -> i64 {
        match self {
            Monkey::Val(a) => *a,
            Monkey::Operation(l, r, '+') => l.borrow().val() + r.borrow().val(),
            Monkey::Operation(l, r, '*') => l.borrow().val() * r.borrow().val(),
            Monkey::Operation(l, r, '/') => l.borrow().val() / r.borrow().val(),
            Monkey::Operation(l, r, '-') => l.borrow().val() - r.borrow().val(),
            _ => panic!(),
        }
    }
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        if let Ok(a) = value.parse() {
            Self::Val(a)
        } else {
            let mut a = value.chars().filter(|x| !x.is_ascii_whitespace());
            let l = std::array::from_fn(|_| a.next().unwrap() as u8);
            let sign = a.next().unwrap();
            let r = std::array::from_fn(|_| a.next().unwrap() as u8);
            Self::Null(l, r, sign)
        }
    }
}

impl Run for Aoc<2022, 21> {
    fn parta(&self) -> Result<AocResult, AocError> {
        let mut a: HashMap<[u8; 4], Rc<RefCell<Monkey>>> = self
            .lines()
            .map(|x| x.split_once(": ").unwrap())
            .map(|x| {
                (
                    x.0.chars()
                        .map(|x| x as u8)
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                    Rc::new(RefCell::new(Monkey::from(x.1))),
                )
            })
            .collect();
        for (k, v) in &a.clone() {
            let Monkey::Null(l, r, o) = v.borrow().clone() else {
                continue;
            };
            let l = Rc::clone(a.get(&l).unwrap());
            let r = Rc::clone(a.get(&r).unwrap());
            // dbg!(&l);
            // dbg!(&r);
            a.entry(*k).and_modify(|x| {
                x.replace(Monkey::Operation(l, r, o));
            });
        }
        // for i in &a {
        //     for j in i.0 {
        //         print!("{}", *j as char);
        //     }
        //     println!();
        //     dbg!(i.1);
        // }
        Ok(a.get(&[b'r', b'o', b'o', b't'])
            .map(|x| x.borrow().val())
            .unwrap()
            .into())
    }
    fn partb(&self) -> Result<AocResult, AocError> {
        let mut a: HashMap<[u8; 4], Rc<RefCell<Monkey>>> = self
            .lines()
            .map(|x| x.split_once(": ").unwrap())
            .map(|x| {
                (
                    x.0.chars()
                        .map(|x| x as u8)
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                    Rc::new(RefCell::new(Monkey::from(x.1))),
                )
            })
            .collect();
        for (k, v) in &a.clone() {
            let Monkey::Null(l, r, o) = v.borrow().clone() else {
                continue;
            };
            let l = Rc::clone(a.get(&l).unwrap());
            let r = Rc::clone(a.get(&r).unwrap());
            // dbg!(&l);
            // dbg!(&r);
            a.entry(*k).and_modify(|x| {
                x.replace(Monkey::Operation(l, r, o));
            });
        }
        let humn = a.get(&[b'h', b'u', b'm', b'n']).unwrap().clone();
        let root = a.get(&[b'r', b'o', b'o', b't']).unwrap().clone();
        let mut i = 0i64;
        let mut change_by = 10000000000;
        let mut old_val = None;
        let ret = loop {
            humn.replace(Monkey::Val(i));
            let new_val = root.borrow().eq();
            if new_val == 0 {
                break i;
            }
            if let Some(a) = old_val.replace(new_val) {
                if a.signum() != new_val.signum() {
                    change_by /= -100;
                }
                i += change_by;
            } else {
                i += change_by;
            }
        };
        // for i in &a {
        //     for j in i.0 {
        //         print!("{}", *j as char);
        //     }
        //     println!();
        //     dbg!(i.1);
        // }
        Ok(ret.into())
    }
}
