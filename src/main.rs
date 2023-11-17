use futures::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::{Client, Collection, Database};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let uri = "mongodb+srv://Gautham:abcd@rustquickstart.ykixlj6.mongodb.net/?retryWrites=true&w=majority"; // Change this to your MongoDB URI

    let client = Client::with_uri_str(uri).await?;

    let database_name = "sai";
    let db: Database = client.database(database_name);

    let collection_name = "Student_Information";

    let collection: Collection<Document> = db.collection(collection_name);

    let student_to_insert = Student {
        name: "Gautham".to_string(),
        age: 27,
    };

    insert_student(&collection, &student_to_insert).await?;

    println!("Student added to the 'Student_Information' collection.");

    let student_to_update = Student {
        name: "Gautham".to_string(),
        age: 26,
    };

    update_student_age(&collection, &student_to_update).await?;
    println!("Student's age updated in the 'Student_Information' collection.");

    let students = find_students(&collection, "Gautham").await?;
    println!("Students in the 'Student_Information' collection:");
    for student in students {
        println!("Name: {}, Age: {}", student.name, student.age);
    }

    let student_to_delete = "Aswin";
    delete_student(&collection, student_to_delete).await?;
    println!(
        "Student '{}' deleted from the 'Student_Information' collection.",
        student_to_delete
    );

    Ok(())
}

struct Student {
    name: String,
    age: i32,
}

async fn insert_student(
    collection: &Collection<Document>,
    student: &Student,
) -> Result<(), Box<dyn Error>> {
    let student_doc = doc! {
        "name": &student.name,
        "age": student.age,
    };

    collection.insert_one(student_doc, None).await?;
    Ok(())
}

async fn update_student_age(
    collection: &Collection<Document>,
    student: &Student,
) -> Result<(), Box<dyn Error>> {
    let filter = doc! { "name": &student.name };
    let update = doc! { "$set": { "age": student.age } };

    collection.update_one(filter, update, None).await?;
    Ok(())
}
async fn find_students(
    collection: &Collection<Document>,
    student_name: &str,
) -> Result<Vec<Student>, Box<dyn Error>> {
    let filter = doc! { "name": student_name };
    let mut students = vec![];

    let mut cursor = collection.find(filter, None).await?;

    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(document) => {
                let name = document.get_str("name")?;
                let age = document.get_i32("age")?;

                students.push(Student {
                    name: name.to_string(),
                    age,
                });
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    Ok(students)
}

async fn delete_student(
    collection: &Collection<Document>,
    student_name: &str,
) -> Result<(), Box<dyn Error>> {
    let filter = doc! { "name": student_name };

    collection.delete_one(filter, None).await?;
    Ok(())
}
