#!/bin/bash

_term() {
    echo "Kafka Terminated.";
    kill -TERM "$child" 2>/dev/null
}

trap _term SIGINT SIGTERM

KAFKA_PROPERTIES=/opt/kafka/config/kraft/server.properties;

if [ -z "$KAFKA_HOST_NAME" ]; then
  {
    echo "listeners=CONTROLLER://:19092,EXTERNAL://:9093";
    echo "advertised.listeners=EXTERNAL://localhost:9093";
    echo "inter.broker.listener.name=EXTERNAL";
    echo "listener.security.protocol.map=CONTROLLER:PLAINTEXT,EXTERNAL:PLAINTEXT";
  } >> $KAFKA_PROPERTIES
else
  {
    echo "listeners=CONTROLLER://:19092,INTERNAL://:9092,EXTERNAL://:9093";
    echo "advertised.listeners=INTERNAL://${KAFKA_HOST_NAME}:9092,EXTERNAL://localhost:9093";
    echo "inter.broker.listener.name=EXTERNAL";
    echo "listener.security.protocol.map=CONTROLLER:PLAINTEXT,INTERNAL:PLAINTEXT,EXTERNAL:PLAINTEXT";
  } >> $KAFKA_PROPERTIES
fi

KAFKA_UUID=$(./bin/kafka-storage.sh random-uuid);

export KAFKA_UUID

./bin/kafka-storage.sh format -t "$KAFKA_UUID" -c ./config/kraft/server.properties;

./bin/kafka-server-start.sh ./config/kraft/server.properties & child=$!

wait "$child";