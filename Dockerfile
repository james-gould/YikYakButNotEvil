FROM alpine
WORKDIR /am
ADD /target/release/tcp_server /am/tcp_server
ENTRYPOINT ["tcp_server"]
