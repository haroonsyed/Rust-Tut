pub fn run() {
  let arr1 = [1, 2, 3];
  let arr2 = arr1;

  println!("Array values: {:?}", (arr1, arr2)); //Works great

  //Now lets try that with a non-primitive
  let vec1: Vec<i32> = vec![1, 2, 3];
  let vec2 = vec1;

  // println!("Array values: {:?}",(vec1,vec2)); //Complains about the use of a moved value!

  // Instead use a reference explicitly to indicate continued ownership in vec1
  let vec3: Vec<i32> = vec![1, 2, 3];
  let vec4 = &vec3;

  println!("Array values: {:?}", (&vec3, vec4)); //Works, note how & is needed here to
                                                 // indicate continued ownership of vec3
}
