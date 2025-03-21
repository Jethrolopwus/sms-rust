use std::fmt;

#[derive(Debug, Clone)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub grade: String,
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Student ID: {}, Name: {}, Age: {}, Grade: {}",
            self.id, self.name, self.age, self.grade
        )
    }
}

#[derive(Debug)]
pub enum SchoolError {
    StudentNotFound,
    DuplicateStudent,
}

impl fmt::Display for SchoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchoolError::StudentNotFound => write!(f, "Student not found"),
            SchoolError::DuplicateStudent => write!(f, "Student with this ID already exists"),
        }
    }
}

pub struct School {
    students: Vec<Student>,
}

impl School {
    pub fn new() -> Self {
        School {
            students: Vec::new(),
        }
    }

    // ======Create a new student=======//
    pub fn add_student(&mut self, student: Student) -> Result<(), SchoolError> {
        if self.students.iter().any(|s| s.id == student.id) {
            return Err(SchoolError::DuplicateStudent);
        }
        self.students.push(student);
        Ok(())
    }

    // =====Read all students=============//
    pub fn get_all_students(&self) -> &Vec<Student> {
        &self.students
    }

    //========Read a specific student by ID============//
    pub fn get_student_by_id(&self, id: u32) -> Result<&Student, SchoolError> {
        self.students
            .iter()
            .find(|s| s.id == id)
            .ok_or(SchoolError::StudentNotFound)
    }

    //============== Update a student's details============//
    pub fn update_student(
        &mut self,
        id: u32,
        name: String,
        age: u8,
        grade: String,
    ) -> Result<(), SchoolError> {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.name = name;
            student.age = age;
            student.grade = grade;
            Ok(())
        } else {
            Err(SchoolError::StudentNotFound)
        }
    }

    //=================== Delete a student by ID================//
    pub fn delete_student(&mut self, id: u32) -> Result<(), SchoolError> {
        if self.students.iter().any(|s| s.id == id) {
            self.students.retain(|s| s.id != id);
            Ok(())
        } else {
            Err(SchoolError::StudentNotFound)
        }
    }
}
