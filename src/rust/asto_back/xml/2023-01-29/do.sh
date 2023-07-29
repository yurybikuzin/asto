#!/usr/bin/env bash
dir=$(dirname "$0")
cd "$dir"
xsltproc --output "s6.xml" "../copy.xsl" "Результаты СВД+СММ.xml"
xsltproc --output "svd.xml" "../copy.xsl" "Результаты СВД.xml"
xsltproc --output "smm.xml" "../copy.xsl" "Результаты СММ.xml"
