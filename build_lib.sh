#!/bin/sh

docker run -it -v $(pwd):/ws --net=host microros/micro_ros_static_library_builder:humble
