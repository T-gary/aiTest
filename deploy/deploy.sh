#!/usr/bin/env bash
# 部署到 192.168.1.1:/home/chengzhen/桌面/教学/project/1/pj
# 用法：先在本机构建前端，再执行本脚本
set -euo pipefail

HOST=${HOST:-root@192.168.1.1}
DEST=${DEST:-/home/chengzhen/桌面/教学/project/1/pj}
ROOT=$(cd "$(dirname "$0")/.." && pwd)

(cd "$ROOT/web" && npm run build)

ssh "$HOST" "mkdir -p '$DEST'"
scp -r "$ROOT/web/dist" "$ROOT/server" "$ROOT/README.md" "$ROOT/AGENTS.md" "$HOST:$DEST/"

ssh "$HOST" "bash -lc 'cd \"$DEST/server\" && cargo build --release'"
echo "完成。服务器上启动："
echo "  DATABASE_URL=mysql://root:123456@127.0.0.1:3306/pj_shop PORT=5050 $DEST/server/target/release/pj-shop"
