#!/bin/sh
# @Author: BlahGeek
# @Date:   2017-06-18
# @Last Modified by:   BlahGeek
# @Last Modified time: 2017-08-09

RES="$(echo "$1" | bc -l)"
echo -e "title:${RES}\x01\n"
echo -e "subtitle:$1\x01\n"
