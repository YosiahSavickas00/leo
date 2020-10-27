// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::TestDynamicCheck;

#[test]
fn test_duplicate_variable() {
    let bytes = include_bytes!("duplicate_variable.leo");
    let check = TestDynamicCheck::new(bytes);

    check.expect_create_error();
}

#[test]
fn test_duplicate_variable_multi() {
    let bytes = include_bytes!("duplicate_variable_multi.leo");
    let check = TestDynamicCheck::new(bytes);

    check.expect_create_error();
}

#[test]
fn test_not_enough_values() {
    let bytes = include_bytes!("not_enough_values.leo");
    let check = TestDynamicCheck::new(bytes);

    check.expect_create_error();
}

#[test]
fn test_too_many_values() {
    let bytes = include_bytes!("too_many_values.leo");
    let check = TestDynamicCheck::new(bytes);

    check.expect_create_error();
}
