fn grades(){
    //create a vector of u8
    let mut grades_vec:Vec<u8> = Vec::new();

    let grades_vec_from_macro = vec![10, 8, 10];
    grades_vec.push(10);
    grades_vec.push(8);
    //grades_vec.push(10.1); //error: mismatched types

    println!("Grades: {:?}", grades_vec);
    println!("Grades from macro: {:?}", grades_vec_from_macro);

    println!("Grade at index 0: {}", grades_vec[0]);

    //uses a match expression to handle the case where the index is out of bounds
    println!("Grade at index 3: {}", match grades_vec.get(3){
        Some(grade) => grade,
        None => &0
    });
}

fn main(){
    grades();
}