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

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use crate::{
    ast::{BinaryOperator, Expr, LockTableType, LockTables, MySqlTableLock, Statement},
    dialect::Dialect,
    keywords::Keyword,
    parser::{Parser, ParserError},
};

use super::keywords;

const RESERVED_FOR_TABLE_ALIAS_MYSQL: &[Keyword] = &[
    Keyword::USE,
    Keyword::IGNORE,
    Keyword::FORCE,
    Keyword::STRAIGHT_JOIN,
];

/// A [`Dialect`] for [MySQL](https://www.mysql.com/)
#[derive(Debug)]
pub struct MySqlDialect {}

impl Dialect for MySqlDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        // See https://dev.mysql.com/doc/refman/8.0/en/identifiers.html.
        // Identifiers which begin with a digit are recognized while tokenizing numbers,
        // so they can be distinguished from exponent numeric literals.
        ch.is_alphabetic()
            || ch == '_'
            || ch == '$'
            || ch == '@'
            || ('\u{0080}'..='\u{ffff}').contains(&ch)
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        self.is_identifier_start(ch) || ch.is_ascii_digit()
    }

    fn is_delimited_identifier_start(&self, ch: char) -> bool {
        ch == '`'
    }

    fn identifier_quote_style(&self, _identifier: &str) -> Option<char> {
        Some('`')
    }

    // See https://dev.mysql.com/doc/refman/8.0/en/string-literals.html#character-escape-sequences
    fn supports_string_literal_backslash_escape(&self) -> bool {
        true
    }

    fn ignores_wildcard_escapes(&self) -> bool {
        true
    }

    fn supports_numeric_prefix(&self) -> bool {
        true
    }

    fn parse_infix(
        &self,
        parser: &mut crate::parser::Parser,
        expr: &crate::ast::Expr,
        _precedence: u8,
    ) -> Option<Result<crate::ast::Expr, ParserError>> {
        // Parse DIV as an operator
        if parser.parse_keyword(Keyword::DIV) {
            Some(Ok(Expr::BinaryOp {
                left: Box::new(expr.clone()),
                op: BinaryOperator::MyIntegerDivide,
                right: Box::new(parser.parse_expr().unwrap()),
            }))
        } else {
            None
        }
    }

    fn parse_statement(&self, parser: &mut Parser) -> Option<Result<Statement, ParserError>> {
        if parser.parse_keywords(&[Keyword::LOCK, Keyword::TABLE]) {
            Some(parse_lock_tables(parser, false))
        } else if parser.parse_keywords(&[Keyword::LOCK, Keyword::TABLES]) {
            Some(parse_lock_tables(parser, true))
        } else if parser.parse_keywords(&[Keyword::UNLOCK, Keyword::TABLE]) {
            Some(parse_unlock_tables(parser, false))
        } else if parser.parse_keywords(&[Keyword::UNLOCK, Keyword::TABLES]) {
            Some(parse_unlock_tables(parser, true))
        } else {
            None
        }
    }

    fn require_interval_qualifier(&self) -> bool {
        true
    }

    fn supports_limit_comma(&self) -> bool {
        true
    }

    /// See: <https://dev.mysql.com/doc/refman/8.4/en/create-table-select.html>
    fn supports_create_table_select(&self) -> bool {
        true
    }

    /// See: <https://dev.mysql.com/doc/refman/8.4/en/insert.html>
    fn supports_insert_set(&self) -> bool {
        true
    }

    fn supports_user_host_grantee(&self) -> bool {
        true
    }

    fn is_table_factor_alias(&self, explicit: bool, kw: &Keyword, _parser: &mut Parser) -> bool {
        explicit
            || (!keywords::RESERVED_FOR_TABLE_ALIAS.contains(kw)
                && !RESERVED_FOR_TABLE_ALIAS_MYSQL.contains(kw))
    }

    fn supports_table_hints(&self) -> bool {
        true
    }

    fn requires_single_line_comment_whitespace(&self) -> bool {
        true
    }

    fn supports_match_against(&self) -> bool {
        true
    }

    fn supports_set_names(&self) -> bool {
        true
    }

    fn supports_comma_separated_set_assignments(&self) -> bool {
        true
    }
}

/// `LOCK TABLES`
/// <https://dev.mysql.com/doc/refman/8.0/en/lock-tables.html>
fn parse_lock_tables(
    parser: &mut Parser,
    pluralized_table_keyword: bool,
) -> Result<Statement, ParserError> {
    let tables = parser.parse_comma_separated(parse_lock_table)?;
    Ok(Statement::LockTables(LockTables::MySql {
        pluralized_table_keyword,
        tables,
    }))
}

// tbl_name [[AS] alias] lock_type
fn parse_lock_table(parser: &mut Parser) -> Result<MySqlTableLock, ParserError> {
    let table = parser.parse_object_name(false)?;
    let alias =
        parser.parse_optional_alias(&[Keyword::READ, Keyword::WRITE, Keyword::LOW_PRIORITY])?;
    let lock_type = parse_lock_tables_type(parser)?;

    Ok(MySqlTableLock {
        table,
        alias,
        lock_type: Some(lock_type),
    })
}

// READ [LOCAL] | [LOW_PRIORITY] WRITE
fn parse_lock_tables_type(parser: &mut Parser) -> Result<LockTableType, ParserError> {
    if parser.parse_keyword(Keyword::READ) {
        if parser.parse_keyword(Keyword::LOCAL) {
            Ok(LockTableType::Read { local: true })
        } else {
            Ok(LockTableType::Read { local: false })
        }
    } else if parser.parse_keyword(Keyword::WRITE) {
        Ok(LockTableType::Write {
            low_priority: false,
        })
    } else if parser.parse_keywords(&[Keyword::LOW_PRIORITY, Keyword::WRITE]) {
        Ok(LockTableType::Write { low_priority: true })
    } else {
        parser.expected("an lock type in LOCK TABLES", parser.peek_token())
    }
}

/// UNLOCK TABLES
/// <https://dev.mysql.com/doc/refman/8.0/en/lock-tables.html>
fn parse_unlock_tables(
    _parser: &mut Parser,
    pluralized_table: bool,
) -> Result<Statement, ParserError> {
    Ok(Statement::UnlockTables(pluralized_table))
}
