fn main() {
    println!("Arrays are fixed length lists of values of the same type");
    println!("Arrays have a iterator");
    let grades: [f32; 4] = [10.0, 8.0, 9.5, 6.0];
    println!("The first grade is {}", grades[0]);
    println!("The second grade is {}", grades[1]);
    println!("The third grade is {}", grades[2]);
    println!("The fourth grade is {}", grades[3]);

    for grade in grades {
        println!("The grade is {}", grade);
    }

    for i in 0..grades.len() {
        println!("The grade at index {} is {}", i, grades[i]);
    }

    println!("We have {} grades", grades.len());

    println!("We can also create arrays with the same value");
    let _equal_grades: [f32; 4] = [10.0; 4];

    println!("We can also create matrices");
    let matrix: [[f32; 3]; 2] = [
        [0.2, 1.2, 0.1],
        [1.3, 0.3, 1.4]
    ];

    for line in matrix {
        for value in line {
            print!("{} ", value);
        }
        println!("");
    }

    println!("To acess a value in a array we use usize")
    let index: usize = 1;
    println!("{}", matrix[index][index])
}