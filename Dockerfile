FROM microros/micro_ros_static_library_builder:humble

ENV MICROROS_LIBRARY_FOLDER extras

RUN git clone https://github.com/micro-ROS/micro_ros_arduino.git /project

RUN echo "" >> /entrypoint.sh
RUN echo "mkdir -p /ws/lib" >> /entrypoint.sh
RUN echo "mv /uros_ws/firmware/build/libmicroros.a /ws/lib" >> /entrypoint.sh
