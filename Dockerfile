FROM ubuntu:22.04 as base

RUN sed -i 's/archive.ubuntu.com/mirrors.aliyun.com/g' /etc/apt/sources.list
RUN apt update
# 安装grpc rust运行环境
RUN apt install -y protobuf-compiler libprotobuf-dev

ARG PERSON_CENTER=person-center
ARG FRONTEND_BASE_SERVICE=frontend-base-service
ARG AGGREGATION_GATEWAY=aggregation-gateway

WORKDIR /app

FROM base as person-center

COPY target/release/${PERSON_CENTER} .
RUN chmod +x ${PERSON_CENTER}

EXPOSE 50000

CMD ["./${PERSON_CENTER}"]

FROM base as frontend-base-service

COPY target/release/${FRONTEND_BASE_SERVICE} .
RUN chmod +x ${FRONTEND_BASE_SERVICE}

EXPOSE 50000

CMD ["./${FRONTEND_BASE_SERVICE}"]

FROM base as aggregation-gateway

COPY target/release/${AGGREGATION_GATEWAY} .
RUN chmod +x ${AGGREGATION_GATEWAY}

EXPOSE 80

CMD ["./${AGGREGATION_GATEWAY}"]
