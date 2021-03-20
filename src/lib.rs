use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ExpResult {
    pub descr: String,
    pub path: Vec<String>,
    pub index: Vec<usize>,
    pub met_flag: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CardResult {
    pub met_flag: bool,
    pub stmts: Vec<StatementResult>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StatementResult {
    pub title: String,
    pub met_flag: bool,
    pub exp: Vec<ExpResult>,
}

pub struct Brick<T> {
    pub title: String,
    pub input: T,
    pub expcheck: Box<dyn Fn(T) -> ExpResult + 'static>,
}

impl<T> std::fmt::Debug for Brick<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.title)
    }
}

impl<T> Brick<T> {
    pub fn report(self) -> ExpResult {
        (self.expcheck)(self.input)
    }
}

#[derive(Debug)]
pub struct Statement<T> {
    pub title: String,
    pub contents: Vec<Brick<T>>,
}

impl<T> Statement<T> {
    pub fn report(self) -> StatementResult {
        let mut met_flag = false;
        let mut results = Vec::new();
        for brick in self.contents {
            let out = brick.report();
            if out.met_flag {
                met_flag = true;
            }
            results.push(out)
        }
        StatementResult {
            title: self.title,
            met_flag: met_flag,
            exp: results,
        }
    }
}

#[derive(Debug)]
pub struct Card<T> {
    pub statements: Vec<Statement<T>>,
}

impl<T> Card<T> {
    pub fn report(self) -> CardResult {
        let mut met_flag = true;
        let mut results = Vec::new();
        for statement in self.statements {
            let stmt = statement.report();
            if !stmt.met_flag {
                met_flag = false;
            }
            results.push(stmt);
        }
        CardResult {
            stmts: results,
            met_flag: met_flag,
        }
    }
}

pub trait Config<T> {
    fn expression_function(&self) -> Box<dyn Fn(T) -> ExpResult + 'static>;
    fn name(&self) -> String;
}

impl<T> std::fmt::Debug for dyn Config<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
