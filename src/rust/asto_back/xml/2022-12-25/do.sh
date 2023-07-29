#!/usr/bin/env bash
dir=$(dirname "$0")
cd "$dir"
xsltproc --output "s6.xml" "../copy.xsl" "результаты 25_12_2022.xml"
