FROM hlf01/cryptg:cryptg0.4.0-python3.8.16-alpine3.17 AS cryptg_builder
FROM python:3.8.16-alpine3.17
WORKDIR /telegram-onedrive
COPY ./ ./
RUN apk add --update --no-cache libgcc git &&\
    pip install --no-cache-dir telethon requests flask onedrivesdk==1.1.8 git+https://github.com/hlf20010508/LTorrent.git@1.6.0#subdirectory=ltorrent_async &&\
    apk del git
COPY --from=cryptg_builder /cryptg /usr/local/lib/python3.8/site-packages
FROM alpine:3.15.4

RUN apk update
RUN apk add --no-cache git
RUN apk add --no-cache docker-engine
RUN apk add --no-cache docker-cli
RUN apk add --no-cache docker-compose

ADD ./telegram-onedrive /main.py

EXPOSE 8000
EXPOSE 35729
EXPOSE 3000
EXPOSE 1313
