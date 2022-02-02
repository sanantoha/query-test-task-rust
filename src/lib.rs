use std::cmp::Ordering;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::collections::{HashMap, BTreeMap};
use std::ops::Bound::{Unbounded, Excluded};

const ACCURACY: f64 = 1_000_000_000.0;


pub fn select(t1: &Path, t2: &Path, t3: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>>{    
    let vec1 = read_file(t1)?;
    let vec2 = read_file(t2)?;
    let vec3 = read_file(t3)?;

    let vec23 = cartesian_join(&vec2, &vec3);
    // println!("{:?}", vec23);
    let vec23_grouped = group_by(&vec23);
    // println!("{:?}", vec23_grouped);

    let vec_joined = left_join(&vec1, &vec23_grouped);

    let mut vec_joined_grouped = group_by(&vec_joined);
    // println!("{:?}", vec_joined);

    let res = sort_and_limit(&mut vec_joined_grouped, 10);
    // println!("{:?}", res);

    write_file(output, &res)
}

fn left_join(vec1: &Vec<(f64, f64)>, vec2: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let mut map: BTreeMap<u64, &(f64, f64)> = BTreeMap::new();
    for item in vec2.iter() {
        let key = float_to_uint(&item.0);
        map.insert(key, item);
    }
    
    let mut vec: Vec<(f64, f64)> = Vec::new();

    for (fst1, snd1) in vec1.iter() {
        let key = float_to_uint(fst1);
        let mut is_element_in_range = false;
        for (_, (_, snd2)) in map.range((Excluded(&key), Unbounded)) {
            vec.push((*fst1, snd1 * snd2));
            is_element_in_range = true;
        }
        if !is_element_in_range {
            vec.push((*fst1, 0.0));
        }
    }
        
    vec
}

fn group_by(vec: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let mut map: HashMap<u64, f64> = HashMap::new();    

    for (fst, snd) in vec.iter() {
        let key = float_to_uint(fst);
        
        match map.get_mut(&key) {
            Some(v) => *v += snd,
            None => {
                map.insert(key, *snd);
            }
        }
    }
    
    Vec::from_iter(map.iter()).iter()
        .map(|(fst, snd)| ((**fst as f64) / ACCURACY, **snd)).collect()
}

fn float_to_uint(x: &f64) -> u64 {
    (x * ACCURACY).trunc() as u64
}

fn sort_and_limit(vec: &mut Vec<(f64, f64)>, n: usize) -> Vec<(f64, f64)> {

    vec.sort_by(|(fst1, snd1), (fst2, snd2)| {        
        if snd1.eq(snd2) {
            if fst1.lt(fst2) { return Ordering::Less; }
            else if fst1.eq(fst2) { return Ordering::Equal; }
            else { return Ordering::Greater; }
        } else if snd1.le(snd2) {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    });

    
    vec.iter().take(n).copied().collect()
}

fn cartesian_join(vec1: &Vec<(f64, f64)>, vec2: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let mut res = vec![];
    
    for (b, y) in vec1.iter() {
        for (c, z) in vec2.iter() {
            res.push((b + c, y * z));
        }
    }

    res
}

fn read_file(path: &Path) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut res: Vec<(f64, f64)> = Vec::new();
    
    for line in reader.lines().skip(1) {
        if let Ok(line) = line {
            let mut iter = line.split_whitespace();
            let fst: f64 = iter.next().unwrap().parse()?;
            let snd: f64 = iter.next().unwrap().parse()?;
            res.push((fst, snd));
        }
    }
    
    Ok(res)
}

fn write_file(path: &Path, vec: &Vec<(f64, f64)>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;

    let mut writer = BufWriter::new(file);
    writer.write(format!("{}\n", vec.len()).as_bytes())?;

    for (fst, snd) in vec.iter() {
        writer.write(format!("{} {}\n", fst, snd).as_bytes())?;
    }

    writer.flush()?;

    Ok(())
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cartesian_join() {
        let vec1: Vec<(f64, f64)> = vec![(1.0, 1.0), (2.0, 2.0), (3.0, 3.0)];
        let vec2: Vec<(f64, f64)> = vec![(4.0, 4.0), (5.0, 5.0)];

        let actual = cartesian_join(&vec1, &vec2);
        let expected = vec![(5.0, 4.0), (6.0, 5.0), (6.0, 8.0), (7.0, 10.0), (7.0, 12.0), (8.0, 15.0)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_float_to_uint() {
        let v = 3.141516;
        let actual = float_to_uint(&v);

        assert_eq!(actual, 3141516000);
    }
}