// // Rust functional style
// fn main() {
//   let i: Vec<i32>  = (1..4).collect::<Vec<i32>>();
//   let i_iter:std::slice::Iter<i32> = i.iter();
//   println!("i_iter {:?}",i_iter);
//   let j: Vec<i32>  = (1..3).collect::<Vec<i32>>();
//   let j_iter:std::slice::Iter<i32> = j.iter();
//   println!("j_iter {:?}",j_iter);
  
//   // let two_d_vec :  Vec<Vec<(&i32,&i32)>> = i_iter.clone().map(|i: &i32| j_iter.clone().map(|j: &i32| (i,j)).collect() ).collect();

//   let two_d_vec :  Vec<Vec<(&i32,&i32)>> = i_iter.clone().map(|i: &i32| (1..3).collect::<Vec<i32>>().iter().clone().map(|j: &i32| (i,j)).collect() ).collect();
  
//   println!("two_d_vec {:?}",two_d_vec);
//   // println!("i {:?}",i);
//   println!("i_iter {:?}",i_iter);

// }