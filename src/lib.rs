#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectMode {
    // `>` : create/overwrite the file (truncate to empty first)
    Truncate,
    // `>>` : create if missing, otherwise append to the end
    Append,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
    pub stdin: Option<String>,
    pub stdout: Option<(String, RedirectMode)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pipeline {
    // The stages of the pipeline, left to right. Always at least one stage
    pub stages: Vec<Command>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyCommand,
    // Pipeline stage had no program
    EmptyStage,
    // A redirection operator (`<`, `>`, `>>`) had no file name after it
    MissingRedirectTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Builtin {
    /// `cd [dir]`: change the shell's working directory
    Cd,
    /// `source [file]`: execute commands from file in the current shell
    Source,
    /// `exit`: quit the shell
    Exit,
    /// `pwd`:` print the working directory
    Pwd,
}

pub fn as_builtin(name: &str) -> Option<Builtin> {
    match name {
        "cd" => Some(Builtin::Cd),
        "source" => Some(Builtin::Source),
        "exit" => Some(Builtin::Exit),
        "pwd" => Some(Builtin::Pwd),
        _ => None,
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(|token| token.to_string())
        .collect()
}

pub fn parse_command(tokens: &[String]) -> Result<Command, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::EmptyCommand);
    }

    Ok(Command {
        program: tokens[0].clone(),
        args: tokens[1..].to_vec(),
        stdin: None,
        stdout: None,
    })
}

pub fn parse_pipeline(input: &str) -> Result<Pipeline, ParseError> {
    let tokens = tokenize(input);
    let command = parse_command(&tokens)?;

    Ok(Pipeline {
        stages: vec![command],
    })
}
