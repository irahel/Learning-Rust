fn grades(){
    //create a vector of u8
    let mut grades_vec:Vec<u8> = Vec::new();
    let grades_with_capacity = Vec::with_capacity(10);
    let grades_vec_from_macro = vec![10, 8, 10];
    grades_vec.push(10);
    grades_vec.push(9);
    grades_vec.push(8);
    grades_vec.push(7);
    grades_vec.push(6);
    grades_vec.push(5);
    grades_vec.push(4);
    //grades_vec.push(10.1); //error: mismatched types

    println!("Grades: {:?}", grades_vec);
    println!("Grades from macro: {:?}", grades_vec_from_macro);

    println!("Grade at index 0: {}", grades_vec[0]);

    //uses a match expression to handle the case where the index is out of bounds
    println!("Grade at index 3: {}", match grades_vec.get(3){
        Some(grade) => grade,
        None => &0
    });

    for grade in &grades_vec{
        println!("[For]:Grade: {}", grade);
    }

    //pop returns the last element of the vector in a Option, and removes it
    println!("Popped grade: {}", grades_vec.pop().unwrap());

    while let Some(grade) = grades_vec.pop(){
        println!("[While]:Grade: {}", grade);
    }

}

fn main(){
    grades();
}