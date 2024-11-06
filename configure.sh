if [ $# -lt 1 ]; then
  echo "Usage: bash configure.sh <path_to_rc_file>"
  exit 1
fi

mkdir -p ~/.jumptag/bin
cp ./jt-inner ~/.jumptag/bin

~/.jumptag/bin/jt-inner -init "$1"