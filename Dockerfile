FROM microros/micro_ros_static_library_builder:humble

ENV MICROROS_LIBRARY_FOLDER extras

RUN git clone https://github.com/micro-ROS/micro_ros_arduino.git /project

#ENTRYPOINT ["ls", "/project"]
