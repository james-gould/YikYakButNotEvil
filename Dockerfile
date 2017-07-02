FROM debian
ADD /target/release/. /am
CMD ["/am/tcp_server"]
