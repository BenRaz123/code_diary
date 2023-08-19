#![doc(html_logo_url = "https://picsum.photos/150?grayscale")]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod backend;
pub mod diaryentry;
pub mod timestamps;
pub mod frontend;

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

fn main() {
    let args: Cli = argh::from_env();

    let added_from_args: bool = match &args.action {
        Some(Action::Add(Add {title: _title, content})) => content.is_some(),
        _ => false
    };

    let action = match args.action {
        Some(action) => action,
        None => prompt_action()
    };
    
    if let Action::Add( Add {title, content} ) = &action {
        let title: Option<String> = match added_from_args {
            true => title.to_owned(),
            false => match title {
                Some(title) => Some(title.to_owned()),
                None => prompt_for_title(),
            }
        };

        let content: String = match content {
            Some(content) => content.to_owned(),
            None => prompt_for_content(),
        };
    
        let entry = DiaryEntry::from_str_and_option(&title, &content);
        store_entry(&entry);
    }

    if let Action::Delete(Delete {index}) = &action { 
        todo!();
    }

    if let Action::View(View {index}) = &action {
        let entries = get_entries();

        if entries.len() == 0 {
            println!("You have no entries, silly!");
            std::process::exit(1);
        }

        let index: u64 = match index {
            Some(index) => *index,
            None => prompt_for_viewing(&entries),
        };

        println!("{}", entries[index as usize].show_detail());
    }
}    
