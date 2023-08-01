use crate::timestamps::TimeStamp;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// An internal struct consisting of a timestamp, title, body, and a boolean determining whether a
/// title has been manually added by the user
/// ## Example:
/// ```rust
/// use diaryentry::DiaryEntry;
///
/// fn main {
///     // Prompts a user for the entry and stores it.
///     let entry = DiaryEntry::from_prompt();
/// }
/// ```
pub struct DiaryEntry {
    pub date: TimeStamp,
    pub title: Option<String>,
    pub content: String,
}

impl DiaryEntry {
    /// Takes a title and body and generates a timestamp, returning a `DiaryEntry`
    ///
    /// ## Example
    /// ```rust
    ///     use diaryentry::DiaryEntry;
    ///     
    ///     fn main() {
    ///         let hardcoded_diary = DiaryEntry::new(
    ///             "My Decicison",
    ///             "I have decided that IO is too hard. Too much libraries, utitlities, `Results`.
    ///             I will now be hardcoding my user input. My life is so much less stressful!"
    ///         );
    ///         assert_eq!(hardcoded_diary.title.unwrap(), String::from("My Decision"));
    ///     }
    /// ```
    pub fn new<'a>(title: &'a str, content: &'a str) -> Self {
        Self {
            date: TimeStamp::new(),
            title: Some(title.to_string()),
            content: content.to_string(),
        }
    }
    /// Prompts the user for a entry and returns it.
    ///
    /// ## Example
    /// ```rust
    /// use diaryentry::DiaryEntry;
    ///
    /// fn main() {
    ///     let input_entry = DiaryEntry::from_prompt();
    ///     match input_entry {
    ///         DiaryEntry { TimeStamp, Option<String>, String } => println!("yay!"),
    ///         _ => println!("uh oh!")
    ///     }
    /// }
    /// ```
    pub fn from_prompt() -> Self {
        let date = TimeStamp::new();
        let title_question = requestty::Question::input("Title")
            .default("")
            .message("Please give a title (Optional))")
            .build();
        let content = requestty::Question::input("Content")
            .message("Please enter your diary entry")
            .build();

        let title_answer = requestty::prompt_one(title_question).unwrap();
        let title_answer = title_answer.as_string().unwrap();

        let title = match title_answer {
            "" => None,
            _ => Some(title_answer.to_string()),
        };

        let content = requestty::prompt_one(content);

        if content.is_err() {
            eprintln!("Failed to take input!");
            std::process::exit(1);
        }
        if content.as_ref().unwrap().as_string().is_none() {
            eprintln!("Answer could not be taken!");
            std::process::exit(1);
        }
        let content = &content.unwrap().as_string().unwrap().to_string();
        Self {
            date,
            title,
            content: content.clone(),
        }
    }
}

impl std::fmt::Display for DiaryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_title = match &self.title {
            Some(title) => color_print::cformat!("<blue, bold>{title}</>"),
            None => color_print::cformat!("<white>Untitled</>")
        };
        write!(
            f,
            "{}",
            color_print::cformat!(
                "{} <magenta>(</><red>{}</><magenta>)</>:\n\t<yellow>{}</>",
                formatted_title,
                self.date,
                self.content,
            ),
        )
    }
}

