use thiserror::Error;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Error, Debug)]
pub enum ArxError {
    #[error("Null pointer in {0}")]
    NullPtr(&'static str),
    #[error("Null resbuf pointer in {0}")]
    ResbufNullPtr(&'static str),
    #[error("Editor, {0}")]
    EditorErr(&'static str),
    #[error("Resbuf, {0}")]
    ResbufErr(&'static str),
    #[error("LispFun, {0}")]
    LispFunErr(&'static str),
    #[error("Command, {0}")]
    CommandErr(&'static str),
    #[error("Invalid resType, {0}")]
    InvalidResType(&'static str),
    #[error("Object Id, {0}")]
    ObjectIdErr(&'static str),
    #[error("Transaction, {0}")]
    TransactionErr(&'static str),
    #[error("Object, {0}")]
    ObjectErr(&'static str),
    #[error("SelectionSet, {0}")]
    SSNameErr(&'static str),
    #[error("Entity, {0}")]
    ENameErr(&'static str),
    #[error("Curve, {0}")]
    CurveErr(&'static str),
    #[error("XData in {0}")]
    XDataErr(&'static str),
}