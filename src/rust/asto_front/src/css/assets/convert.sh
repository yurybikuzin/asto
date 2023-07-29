#!/usr/bin/env bash

# Prerequisites:
#   sudo apt update && sudo apt install -y imagemagick

dir=$(dirname "$0")
cd "$dir"
# convert logo_вправо_SOC.png -crop 500x270+0+100 -resize 50% logo_вправо_SOC.optimized.png
# convert logo_влево_SOC.png -resize 25% logo_влево_SOC.optimized.png
convert logo.1768.png -resize 4.5% logo.80.png
convert logo.1768.png -resize 1.8% logo.32.png
convert logo.1768.png -resize 0.9% logo.16.png
# convert logo_center.png -resize 30% logo_center.optimized.png
# convert logo_right.png -resize 10% logo_right.optimized.png
# convert logo.png -crop 730x1250+180+420 -resize 10% logo.optimized.png
# convert ../hotels/Crocus-Expo-logo-white.png -crop 321x132+102+76 Crocus-Expo-logo-white.optimized.png

