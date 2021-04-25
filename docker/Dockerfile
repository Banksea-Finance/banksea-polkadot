# Node for banksy Parachains.

FROM phusion/baseimage:0.11
LABEL maintainer "contact@banksy.link"
LABEL description="This is the build dev for banksy. Here we create the binary."

ARG PROFILE=release

RUN mv /usr/share/ca* /tmp && \
	rm -rf /usr/share/*  && \
	mv /tmp/ca-certificates /usr/share/ && \
	rm -rf /usr/lib/python* && \
	useradd -m -u 1000 -U -s /bin/sh -d /banksy-parachain banksy && \
	mkdir -p /banksy-parachain/.local/share/banksy-parachain && \
	chown -R banksy:banksy /banksy-parachain && \
	mkdir -p /data && \
	chown -R banksy:banksy /data && \
	ln -s /banksy-parachain/.local/share/banksy-parachain /data && \
	rm -rf /usr/bin /usr/sbin

USER banksy

COPY --chown=banksy bin /banksy-parachain
RUN chmod uog+x /banksy-parachain/banksy-collator

# 30333 for parachain p2p
# 30334 for relaychain p2p
# 9933 for RPC call
# 9944 for Websocket
# 9615 for Prometheus (metrics)
EXPOSE 30333 30334 9933 9944 9615

VOLUME ["/data"]
CMD ["/banksy-parachain/banksy-collator","--ws-external","--collator","-d","/data","--parachain-id","2419","--","--execution","wasm","--chain","/banksy-parachain/rococo.json"]