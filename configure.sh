if [ $# -lt 1 ]; then
  echo "Usage: bash configure.sh <path_to_rc_file>"
  exit 1
fi

mkdir -p ~/.jumptag/bin
cp ./jumptag ~/.jumptag/bin

chmod +x ~/.jumptag/bin/jumptag
~/.jumptag/bin/jumptag -init "$1"