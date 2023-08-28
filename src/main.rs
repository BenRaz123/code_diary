#![doc(html_logo_url = "https://picsum.photos/150?grayscale")]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod backend;
pub mod diaryentry;
pub mod frontend;
pub mod timestamps;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Interact with diary entries
struct Cli {
    #[argh(subcommand)]
    action: Option<Action>,
}

#[derive(FromArgs, PartialEq, Debug, Clone)]
#[argh(subcommand)]
/// The different actions a user can take
pub enum Action {
    Add(Add),
    Delete(Delete),
    View(View),
}

#[derive(FromArgs, PartialEq, Debug, Clone)]
#[argh(subcommand, name = "add")]
/// Add a entry
pub struct Add {
    /// the title
    #[argh(option, short = 't')]
    title: Option<String>,

    /// the content (if not provided, code_diary will start interactively)
    #[argh(option, short = 'c')]
    content: Option<String>,
}

#[derive(FromArgs, PartialEq, Debug, Clone)]
#[argh(subcommand, name = "delete")]
/// Delete at a given index
pub struct Delete {
    #[argh(positional)]
    /// the index to delete at
    index: Option<u64>,
}

#[derive(FromArgs, PartialEq, Debug, Clone)]
#[argh(subcommand, name = "view")]
/// View entries
pub struct View {
    #[argh(positional)]
    /// the index to view
    index: Option<u64>,
}

use backend::*;
use diaryentry::DiaryEntry;
use frontend::*;

fn add(options: Add) {
    let added_from_args: bool = match &argh::from_env::<Cli>().action {
        Some(Action::Add(Add {
            title: _title,
            content,
        })) => content.is_some(),
        _ => false,
    };

    let Add { title, content } = options;

    let title: Option<String> = match added_from_args {
        true => title.to_owned(),
        false => match title {
            Some(title) => Some(title.to_owned()),
            None => prompt_for_title(),
        },
    };

    let content: String = match content {
        Some(content) => content.to_owned(),
        None => prompt_for_content(),
    };

    let entry = DiaryEntry::from_str_and_option(&title, &content);
    store_entry(&entry);
}

fn delete(options: Delete) {
    let Delete { index } = options;

    let mut entries: Vec<DiaryEntry> = get_entries();
    entries.sort();
    let index = match index {
        Some(index) => index,
        None => prompt_for_deletion(&entries),
    };

    if index >= entries.len() as u64 {
        fail!("Index too big!");
    }

    remove_id(entries[index as usize].id);
}

fn view(options: View) {
    let View { index } = options;

    let entries = get_entries();

    if entries.len() == 0 {
        println!("You have no entries, silly!");
        std::process::exit(1);
    }

    let index: u64 = match index {
        Some(index) => index,
        None => prompt_for_viewing(&entries),
    };

    if index >= entries.len() as u64 {
        fail!("Index too big!");
    }

    println!("{}", entries[index as usize].show_detail());
}

fn main() {
    let args: Cli = argh::from_env();

    let action = match args.action {
        Some(action) => action,
        None => prompt_action(),
    };

    match action {
        Action::Add(options) => add(options),
        Action::Delete(options) => delete(options),
        Action::View(options) => view(options),
    }
}
