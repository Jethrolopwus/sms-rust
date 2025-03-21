use sms_rust::lib::basic::sms;

fn main() {

   
    let mut school = sms::School::new();

  
    let student1 = sms::Student {
        id: 1,
        name: "Jethro".to_string(),
        age: 15,
        grade: "10th".to_string(),
    };
    let student2 = sms::Student {
        id: 2,
        name: "Lopwus".to_string(),
        age: 16,
        grade: "11th".to_string(),
    };

    // ===================Add students to the school==================//
    if let Err(e) = school.add_student(student1) {
        println!("Error adding student1: {}", e);
    }
    if let Err(e) = school.add_student(student2) {
        println!("Error adding student2: {}", e);
    }

    //// ============Read all students=============//
    println!("All Students:");
    for student in school.get_all_students() {
        println!("{}", student);
    }

    // =============  Read a specific student ============//
    match school.get_student_by_id(1) {
        Ok(student) => println!("\nFound Student: {}", student),
        Err(e) => println!("Error: {}", e),
    }

    // ====== Update a student========//
    if let Err(e) = school.update_student(1, "Jethro Smith".to_string(), 16, "11th".to_string()) {
        println!("Error: {}", e);
    }

    //======== Delete a student ===================//
    if let Err(e) = school.delete_student(2) {
        println!("Error: {}", e);
    }

    // ==================== Print all students after updates====================//
    println!("\nUpdated Students:");
    for student in school.get_all_students() {
        println!("{}", student);
    }
}