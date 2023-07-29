#!/usr/bin/env bash
dir=$(dirname "$0")
cd "$dir"
xsltproc --output "s6.xml" "../copy.xsl" "2023-03-19.xml"
