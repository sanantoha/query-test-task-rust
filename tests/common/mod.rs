use std::env::temp_dir;
use uuid::Uuid;
use query_test_task_rust::select;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use assert_approx_eq::assert_approx_eq;


pub fn do_test(test_name: &str, case_num: i8) -> Result<(), Box<dyn std::error::Error>> {

    let actual_str = &format!("{}.tst", Uuid::new_v4());
    let actual_path = Path::new(actual_str);
    let output_path_buf = temp_dir().join(actual_path);
    let output = output_path_buf.as_path();

    let path = &format!("resources/case-{}/", case_num);
    let path_str1 = &format!("{}/t1", path);
    let path_str2 = &format!("{}/t2", path);
    let path_str3 = &format!("{}/t3", path);
    let exp_str = &format!("{}/expected-result", path);

    let t1 = Path::new(path_str1);
    let t2 = Path::new(path_str2);
    let t3 = Path::new(path_str3);
    let expected = Path::new(exp_str);

    select(t1, t2, t3, output)?;

    assert_files(test_name, output, expected)
}

fn assert_files(test_name: &str, actual: &Path, expected: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let actual_file = File::open(actual)?;
    let expected_file = File::open(expected)?;

    let mut actual_reader = BufReader::new(actual_file);
    let mut expected_reader = BufReader::new(expected_file);

    let actual_len = read_length(&mut actual_reader)?;
    let expected_len = read_length(&mut expected_reader)?;
    assert_eq!(actual_len, expected_len, 
        "for test {} files have different length {} != {}", test_name, actual_len, expected_len);

    let mut idx = 0;

    while idx < actual_len {
        let (actual_fst, actual_snd) = read_values(&mut actual_reader)?;
        let (expected_fst, expected_snd) = read_values(&mut expected_reader)?;

        assert_approx_eq!(actual_fst, expected_fst, 1e-8f64);
        assert_approx_eq!(actual_snd, expected_snd, 1e-8f64);

        idx += 1;
    }

    Ok(())
}

fn read_values(reader: &mut BufReader<File>) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let mut iter = line.split_whitespace();

    let fst: f64 = iter.next().unwrap().parse()?;
    let snd: f64 = iter.next().unwrap().parse()?;

    Ok((fst, snd))
}

fn read_length(reader: &mut BufReader<File>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut len_str = String::new();
    reader.read_line(&mut len_str)?;

    let res: i32 = len_str.trim().parse()?;
    Ok(res)
}

// fn read_file(path: &Path) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    
//     let mut res: Vec<(f64, f64)> = Vec::new();
    
//     for line in reader.lines().skip(1) {
//         if let Ok(line) = line {
//             let mut iter = line.split_whitespace();
//             let fst: f64 = iter.next().unwrap().parse()?;
//             let snd: f64 = iter.next().unwrap().parse()?;
//             res.push((fst, snd));
//         }
//     }
    
//     Ok(res)
// }