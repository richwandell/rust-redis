#!/bin/bash

# if we have cygpath then this is cygwin
if hash cygpath 2>/dev/null; then
  PWD=$(cygpath "$PWD" -w)
fi

COMMAND="$@"
if [ "$COMMAND" == "" ]; then
  echo "No arguments"
  exit 0
fi

if [ "$1" == "--build" ]; then
  docker build . --tag redis-clone-valgrind
  shift 1;
  COMMAND="$@"
fi

docker stop redis-clone-valgrind-running
docker rm redis-clone-valgrind-running

docker run -d \
  --mount type=bind,source="$PWD",target=/app \
  --name redis-clone-valgrind-running \
  redis-clone-valgrind tail -f /dev/null

docker exec -d redis-clone-valgrind-running \
  bash -c 'valgrind --tool=callgrind --callgrind-out-file=cg.txt target/release/redis-clone --host 0.0.0.0 --port 6379'

sleep 5
docker exec -it redis-clone-valgrind-running bash -c "redis-cli -h localhost -p 6379 $COMMAND"
docker exec -it redis-clone-valgrind-running bash -c 'callgrind_control --dump && callgrind_annotate cg.txt.1 > /app/callgrind.csv && callgrind_control -k'
docker stop redis-clone-valgrind-running
docker rm redis-clone-valgrind-running



