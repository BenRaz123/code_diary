use crate::diaryentry::DiaryEntry;
use crate::Action;
use crate::Add;
use crate::Delete;
use crate::View;

macro_rules! prompt_list {
    ($name:ident with choices [$choices:expr] with message $msg:expr) => {
        let question = requestty::Question::select($msg)
            .choices($choices.iter().map(|x| x.to_string()))
            .message($msg)
            .build();
        let answer = requestty::prompt_one(question).unwrap();
        let $name = answer.as_list_item().unwrap().index;
    };

    ($name:ident with choices [ $( $choice:expr ),+ ] with message $msg:expr) => {
        prompt_list!($name with choices [vec![$($choice),+]] with message $msg);
    }
}

macro_rules! prompt_text {
    ($name:ident with message $msg:expr) => {
        let question = requestty::Question::input($msg)
            .message($msg)
            .build();
        let answer = requestty::prompt_one(question).unwrap();
        let $name = answer.as_string().unwrap();
    }
}

macro_rules! prompt_confirm {
    ($name:ident with message $msg:expr) => {
        let question = requestty::Question::confirm($msg)
            .message($msg)
            .build();
        let answer = requestty::prompt_one(question).unwrap();
        let $name = answer.as_bool().unwrap();
    }
}

pub fn prompt_action() -> Action {
    prompt_list! { action
        with choices ["Add", "Delete", "View"] 
        with message "What do you want to do?"
    };
    
    const ADD: usize = 0;
    const DELETE: usize = 1;
    const VIEW: usize = 2;

    match action {
        ADD => Action::Add(Add { title:None, content:None }),
        DELETE => Action::Delete(Delete { index:None }),
        VIEW => Action::View(View {index:None}),
        _ => unreachable!(),
    }
}

pub fn prompt_for_title() -> Option<String> {
    prompt_text! {
        title with message 
        "Please enter a title (optional) for your diary entry)" 
    };

    if title.len() == 0 {
        return None;
    }

    Some(title.to_string())
}

pub fn prompt_for_deletion(options: &Vec<DiaryEntry>) -> u64 {
    prompt_list! { index 
        with choices [options] 
        with message "Which item do you want to replace?"
    };

    prompt_confirm!(user_is_sure with message "Are you sure?");

    if !user_is_sure {
        std::process::exit(0);
    }

    index as u64
}

pub fn prompt_for_viewing(options: &Vec<DiaryEntry>) -> u64 {
    prompt_list! { index
        with choices [options] 
        with message "Which item do you want to view?" 
    };
    index as u64
}

pub fn prompt_for_content() -> String {
    prompt_text!(content with message "Please write your diary entry");
    content.to_string()
}
