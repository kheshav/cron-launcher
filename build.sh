#!/bin/bash

if [[ "$#" -eq 0 ]];then
  echo "No flag specified"
  exit 127
fi

while test $# -gt 0; do
  case "$1" in
    --arch)
      shift
      arch=$1
      shift
      ;;
    --version)
      shift
      version=$1
      shift
      ;;
    *)
      echo "$1 is not a recognized flag!!"
      exit 1;
      ;;
  esac
done

mkdir -p /tmp/cron-launcher
cp -rv conf /tmp/cron-launcher
cp -v install.sh /tmp/cron-launcher
cp -v ./target/$(arch)/release/cron-launcher /tmp/cron-launcher
chmod +x /tmp/cron-launcher/cron-launcher

# Generate tar files
echo "Generating tar file"
tar -czvf /tmp/cron-launcher_${version}_${arch%-*-*-*}.tar.gz -C /tmp cron-launcher
