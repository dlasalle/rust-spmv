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
