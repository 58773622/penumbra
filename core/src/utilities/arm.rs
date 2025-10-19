/*
    SPDX-License-Identifier: AGPL-3.0-or-later
    SPDX-FileCopyrightText: 2025 Shomy
*/

#[macro_export]
macro_rules! extract_ptr {
    (u32, $data:expr, $offset:expr) => {{
        if $offset + 4 <= $data.len() {
            u32::from_le_bytes([
                $data[$offset],
                $data[$offset + 1],
                $data[$offset + 2],
                $data[$offset + 3],
            ])
        } else {
            0
        }
    }};

    (u64, $data:expr, $offset:expr) => {{
        if $offset + 8 <= $data.len() {
            u64::from_le_bytes([
                $data[$offset],
                $data[$offset + 1],
                $data[$offset + 2],
                $data[$offset + 3],
                $data[$offset + 4],
                $data[$offset + 5],
                $data[$offset + 6],
                $data[$offset + 7],
            ])
        } else {
            0
        }
    }};
}

pub fn to_thumb_addr(pos: usize, base_addr: u32) -> u32 {
    ((pos as u32) + base_addr) | 1
}
