curl -sLO https://github.com/protocolbuffers/protobuf/releases/download/v22.2/protoc-22.2-linux-x86_64.zip
unzip protoc-22.2-linux-x86_64.zip -d $HOME/.local
export PATH="$PATH:$HOME/.local/bin"

curl -sLO https://github.com/getdozer/dozer/releases/latest/download/dozer-linux-amd64.deb 
dpkg -i dozer-linux-amd64.deb