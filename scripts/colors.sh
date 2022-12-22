# https://stackoverflow.com/questions/27159322/rgb-values-of-the-colors-in-the-ansi-extended-colors-index-17-255
# https://stackoverflow.com/questions/15682537/ansi-color-specific-rgb-sequence-bash
# https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797


BOLD=$(tput bold)

RED='\e[0;31m'
# \x1b[1;33m
# faptul ca avea 1 inainte de ; inseamna ca era bold
YELLOW="\x1b[0;33m"
GREEN="\x1b[0;32m"
CYAN="\033[0;36m"
WHITE="\e[0;37m"
BLUE="\033[0;34m"
MAGENTA="\033[0;35m"
ITALIC="\x1b[3m"
UNDERLINE="\x1b[4m"
# reset color
# ENDC="\033[0m"
ENDC="\x1b[0m"


# Styles.
# Normal="\x1b[0m"
# Bold="\x1b[1m"
# Faint="\x1b[2m"
# Italic="\x1b[3m"
# Underline="\x1b[4m"
# Blink_Slow="\x1b[5m"
# Blink_Rapid="\x1b[6m"
# Inverse="\x1b[7m"
# Conceal="\x1b[8m"
# Crossed_Out="\x1b[9m"
# # Text colors.
# Black="\x1b[30m"
# Red="\x1b[31m"
# Green="\x1b[32m"
# Yellow="\x1b[33m"
# Blue="\x1b[34m"
# Magenta="\x1b[35m"
# Cyan="\x1b[36m"
# White="\x1b[37m"
# # Background colors.
# Bg_Black="\x1b[40m"
# Bg_Red="\x1b[41m"
# Bg_Green="\x1b[42m"
# Bg_Yellow="\x1b[43m"
# Bg_Blue="\x1b[44m"
# Bg_Magenta="\x1b[45m"
# Bg_Cyan="\x1b[46m"
# Bg_White="\x1b[47m"
# # Resets
# NoStyle="\x1b[0m"
# NoUnderline="\x1b[24m"
# NoInverse="\x1b[27m"
# NoColor="\x1b[39m"




function italic() {
    echo "${ITALIC}${@}${ENDC}"
}

function underline() {
    echo "${UNDERLINE}${@}${ENDC}"
}

# inspiration
function red() {
    echo "${RED}$@${ENDC}"
}

function red_bold() {
    echo "${RED}${BOLD}$@${ENDC}"
}

function yellow() {
    echo "${YELLOW}$@${ENDC}"
}


function e_yellow() {
    echo -e "${YELLOW}$@${ENDC}"
}


function yellow_bold() {
    echo "${YELLOW}${BOLD}$@${ENDC}"
}


function green() {
    echo "${GREEN}$@${ENDC}"
}

function e_green() {
    echo -e "${GREEN}$@${ENDC}"
}

function green_bold () {
    echo "${GREEN}${BOLD}$@${ENDC}"
}


function cyan () {
    echo "${CYAN}${BOLD}$1${ENDC}"
}

function white () {
    printf "${WHITE}$@${ENDC}"
}

# this is for prompt
# cuz prompt its stupid
# and wants escaped ansi with zero length
function escaped_white() {
    echo "%{${WHITE}%}${@}%{${ENDC}%}"
}

function white_no_endc () {
    printf "${WHITE}$@"
}

function white_bold () {
    printf "${WHITE}${BOLD}$@${ENDC}"
}



function blue () {
    echo "${BLUE}${BOLD}$1${ENDC}"
}

function magenta () {
    echo "${MAGENTA}${@}${ENDC}"
}

function magenta_bold () {
    echo "${MAGENTA}${BOLD}${1}${ENDC}"
}


# https://stackoverflow.com/questions/1494178/how-to-define-hash-tables-in-bash
function highlight() {
    declare -A fg_color_map
    fg_color_map[black]=30
    fg_color_map[red]=31
    fg_color_map[green]=32
    fg_color_map[yellow]=33
    fg_color_map[blue]=34
    fg_color_map[magenta]=35
    fg_color_map[cyan]=36

    fg_c=$(echo -e "\e[1;${fg_color_map[$1]}m")
    c_rs=$'\e[0m'
    sed -u s"/$2/$fg_c\0$c_rs/g"
}


