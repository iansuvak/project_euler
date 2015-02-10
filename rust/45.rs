// Ugly with all of the (unsafe) unwrapping but it works.
// Also apparently to have Ord I had to manually implement PartialEq and PartialOrd
//
use std::cmp::Ordering;
use std::cmp::Ord;

#[derive(Clone,Debug, Eq)]
struct GonN {
    n: u64,
    idx: u64,
    size: usize,
}

impl Iterator for GonN {
    type Item = GonN;
    fn next(&mut self) -> Option<GonN> {
        self.idx += 1;

        match self.size {
            3 => {self.n = self.idx * ( self.idx + 1) /2;},
            5 => {self.n = self.idx * (3 * self.idx - 1) /2;},
            6 => {self.n = self.idx * (2 * self.idx - 1);},
            _ => {return None;}
        };
        Some(self.clone())
    }
}
impl PartialEq for GonN{
 fn eq(&self, other: &Self)-> bool {
     self.n == other.n
 }
}

impl PartialOrd for GonN{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.n < other.n { Some(Ordering::Less)}
        else if self.n > other.n {Some(Ordering::Greater)}
        else {Some(Ordering::Equal)}
    }
}

impl Ord for GonN {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.n < other.n { Ordering::Less}
        else if self.n > other.n {Ordering::Greater}
        else {Ordering::Equal}
    }
}

fn initialize_gon(x: u64) -> Result<GonN, &'static str> {
    match x {
        3 => Ok(GonN {n: 1, idx:1, size: 3}),
        5 => Ok(GonN {n: 1, idx:1, size: 5}),
        6 => Ok(GonN {n: 1, idx:1, size: 6}),
        _ => Err("Valid inputs are 3,5,6"),
    }
}

fn main () {
    let mut tri = initialize_gon(3).unwrap().next().unwrap();
    let mut penta = initialize_gon(5).unwrap().next().unwrap();
    let mut hexa = initialize_gon(6).unwrap().next().unwrap();
    let mut count = 0;

    loop {
        while penta < hexa {
            penta = penta.next().unwrap();
        }
        while tri < penta {
            tri = tri.next().unwrap();
        }
        if tri == penta && tri == hexa {
            println!("{}", tri.n);
            count +=1;
            if count == 2 {
                break;
            }
        }
        hexa = hexa.next().unwrap();
    }
}
