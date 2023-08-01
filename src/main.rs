#![doc(html_logo_url = "https://picsum.photos/150?grayscale")]
#![warn(missing_docs)]

pub mod backend;
pub mod diaryentry;
pub mod timestamps;

use backend::*;
use diaryentry::DiaryEntry;

fn main() {
    let entries: Vec<DiaryEntry> = get_entries();
    let question = requestty::Question::select("a")
        .choices(entries.iter().map(|x| x.to_string())).build();
    let _answer = requestty::prompt_one(question).unwrap();
}
