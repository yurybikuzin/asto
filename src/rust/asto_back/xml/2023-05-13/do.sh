#!/usr/bin/env bash
dir=$(dirname "$0")
cd "$dir"
# xsltproc --output "svd.xml" "../copy.xsl" "СВД.xml"
# xsltproc --output "smm.xml" "../copy.xsl" "СММ.xml"
# xsltproc --output "s6.xml" "../copy.xsl" "АСТО.xml"
xsltproc --output "s6_edited.xml" "../copy.xsl" "АСТО_edited.xml"
