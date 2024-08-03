use crate::sudoku::Sudoku;
use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
// #[allow(clippy::module_name_repetitions)]
pub enum Error {
    SolveError,
    ParseError,
    IndexError,
    NoSolution {
        num_recursions: i32,
        guesses: i32,
    },
    #[from]
    Io(std::io::Error),
    #[from]
    ParseInt(std::num::TryFromIntError),
    #[from]
    Format(std::fmt::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SolveError => write!(f, "SolveError"),
            Self::ParseError => write!(f, "ParseError"),
            Self::IndexError => write!(f, "IndexError"),
            Self::NoSolution {
                num_recursions,
                guesses,
            } => write!(
                f,
                "NoSolution: num_recursions: {num_recursions}, guesses: {guesses}"
            ),
            Self::Io(e) => write!(f, "Io: {e}"),
            Self::ParseInt(e) => write!(f, "ParseInt: {e}"),
            Self::Format(e) => write!(f, "Format: {e}"),
        }
    }
}

impl From<&Sudoku> for Error {
    fn from(sudoku: &Sudoku) -> Self {
        Self::NoSolution {
            num_recursions: sudoku.num_recursions,
            guesses: sudoku.guesses,
        }
    }
}

impl From<&mut Sudoku> for Error {
    fn from(sudoku: &mut Sudoku) -> Self {
        Self::NoSolution {
            num_recursions: sudoku.num_recursions,
            guesses: sudoku.guesses,
        }
    }
}
