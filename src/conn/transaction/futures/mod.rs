// Copyright (c) 2016 Anatoly Ikorsky
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use conn::Conn;
use conn::futures::BatchExec;
use conn::futures::First;
use conn::futures::FirstExec;
use conn::futures::PrepExec;
use conn::futures::Query;
use conn::futures::query_result::BinQueryResult;
use conn::futures::query_result::TextQueryResult;
use conn::transaction::Transaction;
use lib_futures::Map;


mod commit;
mod rollback;
mod start_transaction;
pub mod query_result;

pub use self::commit::Commit;
pub use self::commit::new as new_commit;

pub use self::rollback::Rollback;
pub use self::rollback::new as new_rollback;

pub use self::start_transaction::StartTransaction;
pub use self::start_transaction::new as new_start_transaction;

pub type TransBatchExec = Map<BatchExec, fn(Conn) -> Transaction>;
pub type TransFirst<R> = Map<First<R>, fn((Option<R>, Conn)) -> (Option<R>, Transaction)>;
pub type TransFirstExec<R> = Map<FirstExec<R>, fn((Option<R>, Conn)) -> (Option<R>, Transaction)>;
pub type TransPrepExec = Map<PrepExec, fn(BinQueryResult) -> self::query_result::TransBinQueryResult>;
pub type TransQuery = Map<Query, fn(TextQueryResult) -> self::query_result::TransTextQueryResult>;