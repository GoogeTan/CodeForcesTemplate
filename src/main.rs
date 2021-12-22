use std::alloc::System;
use std::collections::VecDeque;
use std::str::FromStr;
use std::sync::{LockResult, Mutex};
use std::thread;
use std::thread::{JoinHandle, Thread};

const MULTIPLY: bool = false;

fn solve(index : usize, cin : &mut Cin)
{
    
}

fn main()
{
    let mut cin = Cin::new();
    if MULTIPLY
    {
        let n : usize = cin.get();
        for id in 0..n
        {
            solve(id, &mut cin);
        }
    }
    else
    {
        solve(0, &mut cin)
    }
}


struct Cin
{
    chache: VecDeque<String>
}

impl Cin
{
    fn new() -> Cin
    {
        return Cin { chache : VecDeque::new() }
    }

    fn get<T>(&mut self) -> T where T : FromStr
    {
        if self.chache.is_empty()
        {
            let cin: std::io::Stdin = std::io::stdin();
            let mut line = String::new();
            cin.read_line(&mut line).ok().unwrap();
            for i in line.split(' ')
            {
                self.chache.push_back(i.to_string())
            }
        }
        let res: T = self.chache.front().unwrap().trim().parse().ok().unwrap();
        self.chache.pop_front();
        return res;
    }
}