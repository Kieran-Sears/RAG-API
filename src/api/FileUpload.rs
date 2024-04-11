use rocket::Data;
use rocket::data::Data;
use std::io::Read;

#[post("/upload", format = "json", data = "<data>")]
async fn upload(state: &State<AppState>, data: Data) -> String {
    let mut buffer = String::new();
    data.open().read_to_string(&mut buffer).await.unwrap();
    // Now `buffer` contains the JSON data uploaded
    // Pass it to your "todo" function for processing
    todo!("Process the uploaded JSON data");
}
