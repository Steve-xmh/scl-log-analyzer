mod log_parser;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurrentStatement {
    StartingGame,
    WindowInitialized,
    JoiningSingleplayerGame,
    JoiningMutiplayerGame,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Issues {
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ClientAnalyzer {
    current_statement: CurrentStatement,
    parsing_line: usize,
    parsing_col: usize,
    current_line: usize,
    current_col: usize,
    parsing_log: String,
    issues: Vec<Issues>,
}

impl Default for ClientAnalyzer {
    fn default() -> Self {
        Self {
            current_statement: CurrentStatement::StartingGame,
            parsing_line: 1,
            parsing_col: 0,
            current_line: 1,
            current_col: 0,
            parsing_log: String::with_capacity(1024),
            issues: Vec::with_capacity(16),
        }
    }
}

pub trait Analyzer {
    fn feed_chunk(&mut self, chunk: impl AsRef<str>);
    fn end(&mut self);
    fn issues(&self) -> &[Issues];
    fn current_statement(&self) -> CurrentStatement;
}

impl ClientAnalyzer {
    fn check_log(log: &str, statement: &mut CurrentStatement, _issues: &mut Vec<Issues>) {
        println!("{:?}", log);
        if log.starts_with("[Render thread/INFO]: Backend library:")
            && statement == &CurrentStatement::StartingGame
        {
            *statement = CurrentStatement::WindowInitialized;
        } else if log.starts_with("[Server thread/INFO]: ")
            && log.trim_end().ends_with(" joined the game")
            && statement == &CurrentStatement::WindowInitialized
        {
            *statement = CurrentStatement::JoiningSingleplayerGame;
        } else if log.starts_with("[Render thread/INFO]: Connecting to ")
            && statement == &CurrentStatement::WindowInitialized
        {
            *statement = CurrentStatement::JoiningMutiplayerGame;
        }
    }
}

/// A Minecraft game log analyzer.
impl Analyzer for ClientAnalyzer {
    fn feed_chunk(&mut self, chunk: impl AsRef<str>) {
        // let current_col = self.current_col;
        for c in chunk.as_ref().chars() {
            if c == '\n' {
                self.current_line += 1;
                self.current_col = 0;
            } else {
                self.current_col += 1;
            }
            self.parsing_log.push(c);
        }
        match log_parser::parse_log_line(self.parsing_log.as_str(), true) {
            Ok((rest, log)) => {
                Self::check_log(log, &mut self.current_statement, &mut self.issues);
                self.parsing_line = self.current_line;
                self.parsing_col = self.current_col;
                self.parsing_log = rest.to_string();
            }
            Err(nom::Err::Incomplete(_)) => {}
            Err(err) => {
                panic!("Error on parsing log {}: {:?}", self.parsing_log, err);
            }
        }
    }

    fn end(&mut self) {
        match log_parser::parse_log_line(self.parsing_log.as_str(), false) {
            Ok((rest, log)) => {
                Self::check_log(log, &mut self.current_statement, &mut self.issues);
                self.parsing_line = self.current_line;
                self.parsing_col = self.current_col;
                self.parsing_log = rest.to_string();
            }
            _ => {}
        }
    }

    fn current_statement(&self) -> CurrentStatement {
        self.current_statement
    }

    fn issues(&self) -> &[Issues] {
        &self.issues
    }
}
