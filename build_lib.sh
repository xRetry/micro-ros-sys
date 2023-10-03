#!/bin/sh

docker run -it --rm -v $(pwd):/ws --net=host mros-static -p esp32 ls

