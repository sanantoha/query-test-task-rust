use std::path::Path;
use query_test_task_rust::select;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let case = 3;
    let path_str1 = &format!("resources/case-{}/t1", case);
    let path_str2 = &format!("resources/case-{}/t2", case);
    let path_str3 = &format!("resources/case-{}/t3", case);
    let output_str = &format!("output-{}", case);

    let path1 = Path::new(path_str1);
    let path2 = Path::new(path_str2);
    let path3 = Path::new(path_str3);
    let output = Path::new(output_str);

    select(path1, path2, path3, output)
}