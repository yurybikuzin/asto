#!/usr/bin/env bash
dir=$(dirname "$0")
cd "$dir"
xsltproc --output "s6.xml" "../copy.xsl" "Звездный Бал - 2023 общий.xml"
xsltproc --output "svd.xml" "../copy.xsl" "Звездный Бал - 2023.xml"
xsltproc --output "smm.xml" "../copy.xsl" "Звездный Бал - 2023 (массовый спорт).xml"
