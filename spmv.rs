use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::io::BufRead;

struct CSR
{
  prefix: Vec<u64>,
  columns: Vec<u32>,
  values: Vec<f32>,
}

fn multiply(matrix: &CSR, in_vec: &Vec<f32>, out_vec: &mut Vec<f32>)
{
  let num_rows = matrix.prefix.len()-1;
  for i in 0..num_rows {
    let mut val : f32 = 0.0;
    for j in matrix.prefix[i as usize]..matrix.prefix[i+1 as usize] {
      val += in_vec[matrix.columns[j as usize] as usize] * matrix.values[j as usize];
    }
    out_vec[i] = val;
  }
}

fn load_vector(filename: String) -> Vec<f32>
{
  let mut v: Vec<f32> = Vec::new();
  let file = match File::open(&filename) {
    Err(why) => panic!("Unable to open {}: {}", filename, why.description()),
    Ok(file) => file,
  };

  let buffer = BufReader::new(&file);
  for (num, line) in buffer.lines().enumerate() {
    let l = line.unwrap();
    let r = l.parse::<f32>();
    if r.is_err() {
      panic!("Failed to parse file value from {} at line {}", l, num);
    }

    v.push(r.unwrap());
  }

  return v;
}

fn main()
{
  let mat = CSR{
    prefix: vec![0; 5],
    columns: vec![0; 5],
    values: vec![0.0; 5],
  };

  let x = vec![1.0; 5];
  let mut y = vec![0.0; 5];

  multiply(&mat, &x, &mut y);
}
