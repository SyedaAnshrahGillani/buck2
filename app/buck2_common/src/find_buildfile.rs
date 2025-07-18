/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use buck2_core::fs::paths::file_name::FileName;
use buck2_core::fs::paths::file_name::FileNameBuf;

use crate::file_ops::metadata::SimpleDirEntry;

pub fn find_buildfile<'a>(
    buildfile_candidates: &'a [FileNameBuf],
    dir_listing: &[SimpleDirEntry],
) -> Option<&'a FileName> {
    for candidate in buildfile_candidates {
        for entry in dir_listing {
            if entry.file_name == *candidate {
                return Some(candidate.as_ref());
            }
        }
    }
    None
}
