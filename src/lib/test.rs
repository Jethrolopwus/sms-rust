use super::basic::sms;

#[test]
fn test_add_student() {
    let mut school = sms::School::new();
    let student = sms::Student {
        id: 1,
        name: "Jethro".to_string(),
        age: 15,
        grade: "10th".to_string(),
    };
  
    assert!(school.add_student(student.clone()).is_ok());

    assert!(school.add_student(student).is_err());
}

#[test]
fn test_get_student_by_id() {
    let mut school = sms::School::new();
    let student = sms::Student {
        id: 1,
        name: "Jethro".to_string(),
        age: 15,
        grade: "10th".to_string(),
    };

    school.add_student(student).unwrap();

    assert!(school.get_student_by_id(1).is_ok());

    assert!(school.get_student_by_id(2).is_err());
}

#[test]
fn test_update_student() {
    let mut school = sms::School::new();
    let student = sms::Student {
        id: 1,
        name: "Jethro".to_string(),
        age: 15,
        grade: "10th".to_string(),
    };

    school.add_student(student).unwrap();


    assert!(school
        .update_student(1, "Jethro Smith".to_string(), 16, "11th".to_string())
        .is_ok());

  
    assert!(school
        .update_student(2, "Lopwus".to_string(), 17, "12th".to_string())
        .is_err());
}

#[test]
fn test_delete_student() {
    let mut school = sms::School::new();
    let student = sms::Student {
        id: 1,
        name: "Jethro".to_string(),
        age: 15,
        grade: "10th".to_string(),
    };

    school.add_student(student).unwrap();

    assert!(school.delete_student(1).is_ok());

    assert!(school.delete_student(2).is_err());
}