FROM eclipse-temurin:8u402-b06-jre-ubi9-minimal

ARG SCALA_VERSION="2.13"
ARG KAFKA_VERSION="3.7.0"

ENV KAFKA_HOST_NAME="kafka"

WORKDIR /opt

RUN curl -o kafka.tgz https://downloads.apache.org/kafka/${KAFKA_VERSION}/kafka_${SCALA_VERSION}-${KAFKA_VERSION}.tgz \
    && tar xzf kafka.tgz \
    && mv ./kafka_${SCALA_VERSION}-${KAFKA_VERSION} ./kafka \
    && rm kafka.tgz

WORKDIR /opt/kafka

COPY ./config/server.properties ./config/kraft/
COPY ./docker-entrypoint.sh .

EXPOSE 9092 9093

ENTRYPOINT ["./docker-entrypoint.sh"]