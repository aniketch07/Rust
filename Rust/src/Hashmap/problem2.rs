//Student Grades
// Create a HashMap to store student names as keys and their grades as values. 
// Implement functions to:
// Add a new student and grade.
// Retrieve a student's grade.
// Remove a student from the map.

use std::collections::HashMap;

struct StudentGrades{
    grades:HashMap<String,i32>,
}

impl StudentGrades{
    fn new()-> Self{
        StudentGrades{
            grades:HashMap::new(),
        }
    }

    fn add_student(&mut self ,name:String,grade:i32){
        self.grades.insert(name,grade);
    }

     // Function to retrieve a student's grade by name
     fn get_grade(&mut self, name: &str) -> Option<&i32> {
        self.grades.get(name)
    }

    fn remove_student(&mut self, name: &str) -> Option<i32> {
        self.grades.remove(name)
    }
}

fn main(){
    let mut student_grades = StudentGrades::new();

    // Add some students and their grades
    student_grades.add_student("Alice".to_string(), 90);
    student_grades.add_student("Bob".to_string(), 85);
    student_grades.add_student("Charlie".to_string(), 88);

    if let Some(grade) = student_grades.get_grade("Alice") {
        println!("Alice's grade: {}", grade);
    } else {
        println!("Alice not found");
    }

     // Remove a student and check if removal was successful
     if let Some(removed_grade) = student_grades.remove_student("Bob") {
        println!("Bob was removed with grade: {}", removed_grade);
    } else {
        println!("Bob not found");
    }

    // Try to retrieve a removed student's grade
    if let Some(grade) = student_grades.get_grade("Bob") {
        println!("Bob's grade: {}", grade);
    } else {
        println!("Bob not found");
    }
}