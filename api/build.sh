curl -sLO https://github.com/protocolbuffers/protobuf/releases/download/v22.2/protoc-22.2-linux-aarch_64.zip
# unzip protoc-22.2-linux-aarch_64.zip -d $HOME/.local
# export PATH="$PATH:$HOME/.local/bin"
unzip protoc-22.2-linux-aarch_64.zip -d $HOME/.local 


curl -sLO https://github.com/getdozer/dozer/releases/latest/download/dozer-linux-aarch64.deb
dpkg -i dozer-linux-aarch64.deb

echo "replacing <pg_password> in dozer-config.yaml with PG_PASSWORD"
export PG_PASSWORD=$(cat /usr/src/rust_jobs_api/.supabase_password)
sed -i "s/<pg_password>/$PG_PASSWORD/g" /usr/src/rust_jobs_api/dozer-config.yaml