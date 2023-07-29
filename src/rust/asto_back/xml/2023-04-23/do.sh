#!/usr/bin/env bash
dir=$(dirname "$0")
cd "$dir"
xsltproc --output "svd.xml" "../copy.xsl" "23_04_2023_Классы.xml"
xsltproc --output "smm.xml" "../copy.xsl" "23_04_2023_Начиинающие.xml"
# xsltproc --output "s6_edited.xml" "../copy.xsl" "результат весь.xml"
