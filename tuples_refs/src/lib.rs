#[derive(Debug, PartialEq, Eq)]
pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32 {
    let Student(id, _, _) = student;
    *id
}
 
pub fn first_name(student: &Student) -> &str {
    let Student(_, first_name, _) = student;
    first_name.as_str()
}

pub fn last_name(student: &Student) -> &str {
    let Student(_, _, last_name) = student;
    last_name.as_str()
}
