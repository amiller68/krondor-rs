#!/bin/bash

REMOTE_IPFS="https://ipfs.infura.io:5001"
LOCAL_PORT=3000
AUTH="Authorization: Basic $(echo -n "${IPFS_KEY}:${IPFS_SECRET}" | base64)"
# socat TCP-LISTEN:$LOCAL_PORT,reuseaddr,fork TCP:$REMOTE_IPFS
# while :; do
#     { 
#         echo -ne "HTTP/1.1 200 OK\r\n$AUTH\r\n\r\n";
#         socat - TCP:$REMOTE_IPFS;
#     } | socat - TCP-LISTEN:$LOCAL_PORT,reuseaddr,fork;
# done
socat TCP-LISTEN:$LOCAL_PORT,fork,reuseaddr TCP:$REMOTE_IPFS,crnl2nul="\r\n\r\n" \
    -v -d -d -lf /tmp/socat.log \
    -,crnl2nul="\r\n\r\n" \
    -exec "/bin/sh -c '\
        read line; \
        echo \"$AUTH\"; \
        echo; \
        while read line; do \
            echo \"$line\"; \
        done \
    '"
