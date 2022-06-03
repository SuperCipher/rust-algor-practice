// fn main() {
//   let collectible = (1..6).collect::<Vec<i32>>().into_iter().map(|x|x).collect::<Vec<i32>>();
//   println!("{:?}",collectible)
//   // (1..6).collect::<Vec<i32>>().iter().map(|x|(1..*x+1).collect::<Vec<i32>>().iter().map(|y| println!("({},{})",x,y)));
// }


fn main() {

  let mut result : Vec<(&i32,&i32)> = vec![];

  // (1..6).for_each(|x| println!("({})",x));
  // let fm = (1..6).flat_map(|x| (1..=x)).collect::<Vec<_>>();

  let fm = (1..6).flat_map(|x| (1..=x-1).map(move |y| (y, x))).collect::<Vec<_>>();
  println!("{:?}",fm)

  // (1..6).collect::<Vec<i32>>().iter().for_each(move |x|(1..*x+1).collect::<Vec<i32>>      ().iter().for_each( move |y| result.push((x,y))));

}