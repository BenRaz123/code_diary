use crate::diaryentry::DiaryEntry;
use crate::timestamps::TimeStamp;

macro_rules! iter {
    ($name:expr) => {{
        let mut list: Vec<String> = Vec::new();
        let db = open_db();
        let query = format!("SELECT {} FROM Entries;", $name);
        db.iterate(query, |pairs| {
            let val = pairs[0].1;
            if val.is_none() {
                fail!("Database Error: Could not read value from DB!");
            }
            list.push(val.unwrap().to_string());
            true
        })
        .unwrap();
        list
    }};
}

macro_rules! fail {
    ($msg:expr) => {{
        eprintln!($msg);
        std::process::exit(1);
    }};

    ($msg:expr, $status:expr) => {
        eprintln!($msg);
        std::process::exit($status);
    };
}

fn execute<'a>(query: &'a str) {
    let db = open_db();
    let status = db.execute(query);
    if let Err(e) = status {
        fail!("DataBase Error: {e}")
    }
}

fn initialize_db() {
    let db = open_db();
    let query = "CREATE TABLE IF NOT EXISTS Entries (TimeStamp TEXT, Title TEXT, Content TEXT);";
    db.execute(query).expect("oopsie");
}

fn open_db() -> sqlite::Connection {
    let connection = sqlite::open(format!("{}/code_diary.db", std::env::var("HOME").unwrap()));
    if connection.is_err() {
        fail!("Database Error!");
    }
    connection.unwrap()
}

pub fn store_entry(entry: &DiaryEntry) {
    initialize_db();
    let date = entry.clone().date;
    let title = entry.clone().title.unwrap_or(String::from(""));
    let content = entry.clone().content;

    let query = format!("INSERT INTO Entries VALUES ('{date}', '{title}', '{content}');");
    execute(&query);
}

pub fn get_entries() -> Vec<DiaryEntry> {
    initialize_db();
    let dates = iter!("TimeStamp")
        .iter()
        .map(|x| TimeStamp::from_string(x).unwrap())
        .collect::<Vec<TimeStamp>>();
    let titles = iter!("Title");
    let contents = iter!("Content");

    if !(dates.len() == titles.len() && titles.len() == contents.len()) {
        fail!("Something went wrong!");
    }

    let mut results: Vec<DiaryEntry> = Vec::new();

    for index in 0..dates.len() {
        let date = dates[index].clone();
        let content = contents[index].clone();

        let title = if titles[index] == "".to_string() {
            None
        } else {
            Some(titles[index].clone())
        };

        results.push(DiaryEntry {
            date,
            title,
            content,
        })
    }
    results
}
