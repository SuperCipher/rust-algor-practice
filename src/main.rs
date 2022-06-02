// Rust functional style
fn main() {
  let i: Vec<i32>  = (1..6).collect::<Vec<i32>>();
  let i_iter:std::slice::Iter<i32> = i.iter();
  println!("Collected (0..10) into: {:?}", i);
  let j: Vec<i32>  = (1..6).collect::<Vec<i32>>();
  let j_iter:std::slice::Iter<i32> = j.iter();
  println!("i_iter {:?}",i_iter);

  // let two_d_vec :  Vec<Vec<(&i32,&i32)>> = i_iter.map(|i| j_iter.map(|j| (j,j)).collect() ).collect();
  let two_d_vec :  Vec<Vec<(&i32,&i32)>> = i_iter.clone().map(|i: &i32| j_iter.clone().map(|j: &i32| (j,j)).collect() ).collect();
  println!("two_d_vec {:?}",two_d_vec);
  // println!("i {:?}",i);
  println!("i_iter {:?}",i_iter);


  // let mut xs = vec![4i32, 2, 3];
  // println!("Initial vector: {:?}", xs);


  
  // xs.iter()
  //   .map(|x| 0xff)
  //   .collect();
}