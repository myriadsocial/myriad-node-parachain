FROM ubuntu:20.04@sha256:bffb6799d706144f263f4b91e1226745ffb5643ea0ea89c2f709208e8d70c999
LABEL social.myriad.image.authors="dev@myriad.social"
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
		libssl1.1 \
		ca-certificates && \
  # apt cleanup
    apt-get autoremove -y && \
    apt-get clean && \
    find /var/lib/apt/lists/ -type f -not -name lock -delete; \
  # Create user and set ownership and permissions as required
  useradd -m -u 1001 -U -s /bin/sh -d /home/myriad myriad && \
  # manage folder data
  mkdir -p /home/myriad/.local/share && \
  mkdir /data && \
  chown -R myriad:myriad /data && \
  ln -s /data /home/myriad/.local/share/myriad
# Add binnary to docker image
COPY ./myriad /usr/local/bin
# Add chainspec
COPY ./raw-parachain-chainspec.json /home/myriad/raw-parachain-chainspec.json
# Set to a non-root built-in user
USER myriad
# Set environment variable
ENV RUST_BACKTRACE=1
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/myriad"]
