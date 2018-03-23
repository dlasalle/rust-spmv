use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;
use std::error::Error;
use std::env;

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

fn save_vector(filename: String, vec: &Vec<f32>)
{
  let file = match File::create(&filename) {
    Err(why) => panic!("Failed to save output vector {}: {}", filename, why.description()),
    Ok(file) => file,
  };

  let mut buffer = BufWriter::new(&file);
  for val in vec {
    let try = write!(buffer, "{}\n", val);
    if try.is_err() {
      panic!("Failed while writing output vector {}", filename)
    }
  }
}

fn main()
{
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Invalid arguments - usage: spmv <in vec> <out vec>")
  }

  let vec_in_filename = args[0].clone();
  let vec_out_filename = args[1].clone();

  let mat = CSR{
    prefix: vec![0; 5],
    columns: vec![0; 5],
    values: vec![0.0; 5],
  };

  let x = load_vector(vec_in_filename);
  let mut y = vec![0.0; 5];

  multiply(&mat, &x, &mut y);

  save_vector(vec_out_filename, &y);
}
