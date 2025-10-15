#!/usr/bin/env python3
#
# SPDX-FileCopyrightText: 2025 Shomy
# SPDX-License-Identifier: AGPL-3.0-or-later
#
from parse_da import DAFile, DAType, DA, DAEntryRegion

if __name__ == "__main__":
    import sys
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <da_file>")
        sys.exit(1)

    with open(sys.argv[1], "rb") as f:
        da_raw_data = f.read()

    da_file = DAFile.parse_da(da_raw_data)

    for da in da_file.das:
        da1 = da.get_da1()
        da2 = da.get_da2()
        hw_code = hex(da.hw_code)

        with open(f"da1_{hw_code}.bin", "wb") as f:
            f.write(da1.data[:-da1.sig_len])
            print(f"Wrote da1.bin, size: {len(da1.data[:-da1.sig_len])} bytes")

        with open(f"da2_{hw_code}.bin", "wb") as f:
            f.write(da2.data[:-da2.sig_len])
            print(f"Wrote da2.bin, size: {len(da2.data[:-da2.sig_len])} bytes")

        if da1.sig_len > 0:
            with open(f"da1_{hw_code}.sig", "wb") as f:
                f.write(da1.data[-da1.sig_len:])
                print(f"Wrote da1.sig, size: {da1.sig_len} bytes")

        if da2.sig_len > 0:
            with open(f"da2_{hw_code}.sig", "wb") as f:
                f.write(da2.data[-da2.sig_len:])
                print(f"Wrote da2.sig, size: {da2.sig_len} bytes")

        print(f"Extracted DA for hw_code: {hw_code}")

    print("DA stages extracted successfully.")
