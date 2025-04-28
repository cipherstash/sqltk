// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![warn(clippy::all)]
//! Test SQL syntax specific to PostgreSQL. The parser based on the
//! generic dialect is also tested (on the inputs it can handle).

#[macro_use]
mod test_utils;
use test_utils::*;

use sqltk_parser::dialect::{GenericDialect, PostgreSqlDialect};

// Helpers

fn pg() -> TestedDialects {
    TestedDialects::new(vec![Box::new(PostgreSqlDialect {})])
}

fn pg_and_generic() -> TestedDialects {
    TestedDialects::new(vec![
        Box::new(PostgreSqlDialect {}),
        Box::new(GenericDialect {}),
    ])
}

// Tests

#[test]
fn parse_select_without_projection() {
    pg_and_generic().verified_stmt("SELECT FROM users");
}

#[test]
fn parse_lock_table() {
    pg().verified_stmt("LOCK customers");
    pg().verified_stmt("LOCK TABLE customers");
    pg().verified_stmt("LOCK TABLE ONLY customers");
    pg().verified_stmt("LOCK TABLE ONLY customers IN ACCESS SHARE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers IN ROW SHARE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers IN ROW EXCLUSIVE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers IN SHARE UPDATE EXCLUSIVE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers IN SHARE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers IN SHARE ROW EXCLUSIVE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers IN EXCLUSIVE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers IN ACCESS EXCLUSIVE MODE");
    pg().verified_stmt("LOCK customers, orders");
    pg().verified_stmt("LOCK TABLE customers, orders");
    pg().verified_stmt("LOCK TABLE ONLY customers, orders");
    pg().verified_stmt("LOCK TABLE ONLY customers, orders IN ACCESS SHARE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers, orders IN ROW SHARE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers, orders IN ROW EXCLUSIVE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers, orders IN SHARE UPDATE EXCLUSIVE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers, orders IN SHARE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers, orders IN SHARE ROW EXCLUSIVE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers, orders IN EXCLUSIVE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers, orders IN ACCESS EXCLUSIVE MODE");
    pg().verified_stmt("LOCK TABLE ONLY customers, orders IN ACCESS SHARE MODE NOWAIT");
}
