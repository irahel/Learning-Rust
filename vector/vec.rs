fn grades(){
    //create a vector of u8
    let mut grades_vec:Vec<u8> = Vec::new();

    let grades_vec_from_macro = vec![10, 8, 10];
    grades_vec.push(10);
    grades_vec.push(8);
    //grades_vec.push(10.1); //error: mismatched types

    println!("Grades: {:?}", grades_vec);
    println!("Grades from macro: {:?}", grades_vec_from_macro);
}

fn main(){
    grades();
}