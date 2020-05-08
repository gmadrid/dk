#!/usr/bin/awk -f

BEGIN { printf "CHART\n" }

$0 != "CHART" { printf "%s\n", substr($0, length($0) / 2) }
