#!/usr/bin/env bash

# Define file paths
transactions_qif="$HOME/Downloads/transactions.qif"
transactions_withMemos_qif="$HOME/Downloads/transactions_withMemos.qif"

DEV_MODE=false

# Check if transactions_withMemos.qif exists
if [ -f "$transactions_withMemos_qif" ]; then
    # Transformation has already been done.
    rm "$transactions_withMemos_qif" "$transactions_qif"
elif [ -f "$transactions_qif" ]; then
    # Run cargo if transactions.qif exists
    if test "$DEV_MODE" = true; then
        echo "[Dev Mode]"
        cargo run "$transactions_qif" "$transactions_withMemos_qif"
    else
        ./target/release/ynab_import "$transactions_qif" "$transactions_withMemos_qif"
    fi
fi
