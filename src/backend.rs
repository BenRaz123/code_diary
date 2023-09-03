//! # Backend module for code_diary
//! mostly includes functions and macros related to backend database operations
//! and anything that persists in memory

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

#[macro_export]
/// A convenience macro for exiting with a nonzero status code with a message
/// Example:
/// ```
/// # use crate::backend::fail;
/// # fn main() {
/// fail!("Bye!");
/// unreachable!();
/// # }
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

/// Gets the latest id. What this meaans is that if there are four diary
/// entries being stored, for example, the function will return 4.
pub fn get_latest_id() -> u64 {
    let mut id_list: Vec<u64> = iter!("Id")
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    id_list.sort_by(|b, a| a.cmp(b));
    let id = match id_list.len() {
        0 => 0,
        _ => id_list[0],
    };
    id
}

fn execute<'a>(query: &'a str) {
    let db = open_db();
    let status = db.execute(query);
    if let Err(e) = status {
        fail!("DataBase Error: {e}")
    }
}

/// Clears the database
pub fn initialize_db() {
    let db = open_db();
    let query = "DROP TABLE IF EXISTS Entries; CREATE TABLE IF NOT EXISTS Entries (Id INTEGER, TimeStamp TEXT, Title TEXT, Content TEXT);";
    db.execute(query).expect("oopsie");
}

fn open_db() -> sqlite::Connection {
    let connection = sqlite::open(format!("{}/code_diary.db", std::env::var("HOME").unwrap()));
    if connection.is_err() {
        fail!("Database Error!");
    }
    let connection = connection.unwrap();
    connection.execute("CREATE TABLE IF NOT EXISTS Entries (Id INTEGER, TimeStamp TEXT, Title TEXT, Content TEXT);").unwrap();
    connection
}
/// Removes a selected ID from the entries database.
/// # Panics
/// Will 'gracefully' exit on a database error
/// # Example
/// ```rust
/// # #![allow(unused)]
/// # use crate::backend::{remove_id, store_entry};
/// # fn main() {
/// let entries = [
///     DiaryEntry::new("Hello", "World"),
///     DiaryEntry::new("This entry", "is a mistake"),
/// ];
///
/// // Inserts them into DB
/// entries.for_each(|entry| store_entry(entry));
///
/// // Remove the second entry
/// remove_id(1);
///
/// assert_eq!(get_entries()[0], entries[0]);
/// # }
/// ```
pub fn remove_id(id: u64) {
    let connection = open_db();
    let query = format!("DELETE FROM Entries WHERE Id = {id};");
    let result = connection.execute(query);
    if let Err(e) = result {
        fail!("Could not remove entry: {e}");
    }
}

/// Stores a `DiaryEntry`
/// # Panics
/// Will automatically 'gracefully' exit on a problem opening the DB
/// # Examples
/// ```rust
/// # #![allow(unused)]
/// # use crate::diaryentry::DiaryEntry;
/// # use crate::backend::{store_entry, get_entries};
/// # fn main() {
/// let entry = DiaryEntry::new(
///     "About to bake a cake, looks good!", "never mind"
/// );
/// store_entry(entry);
/// assert_eq!(entry, get_entries()[0]);
/// # }
pub fn store_entry(entry: &DiaryEntry) {
    //initialize_db();
    let date = entry.clone().date;
    let title = entry.clone().title.unwrap_or(String::from(""));
    let content = entry.clone().content;
    let id = entry.clone().id;

    let query = format!("INSERT INTO Entries VALUES ({id}, '{date}', '{title}', '{content}');");
    execute(&query);
}

/// Returns A Vector of `DiaryEntry`s from the database
/// # Panics
/// Will automatically 'gracefully' exit if a problem is encountered
/// # Examples
/// Find out how many Entries are stored the user's diary:
/// ```rust
/// # use crate::backend::get_entries;
/// # use crate::diaryentry::DiaryEntry
/// # #![allow(unused)]
/// # fn main() {
/// let entries: Vec<DiaryEntry> = get_entries();
/// let length = get_entries.len();
/// println!("You have {length} diary entries!");
/// # }
/// ```
pub fn get_entries() -> Vec<DiaryEntry> {
    let ids = iter!("Id")
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
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
        let id = ids[index].clone();
        let date = dates[index].clone();
        let content = contents[index].clone();

        let title = if titles[index] == "".to_string() {
            None
        } else {
            Some(titles[index].clone())
        };

        results.push(DiaryEntry {
            id,
            date,
            title,
            content,
        })
    }
    results
}
