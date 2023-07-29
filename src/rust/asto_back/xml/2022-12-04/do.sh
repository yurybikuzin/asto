#!/usr/bin/env bash
dir=$(dirname "$0")
cd "$dir"
xsltproc --output "svd.xml" "copy.xsl" "СВД.xml"
xsltproc --output "smm.xml" "copy.xsl" "СММ.xml"
