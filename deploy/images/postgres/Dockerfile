# AKR PostgreSQL image: the pinned PostgreSQL 17 plus pgBackRest (archive_command needs the binary beside
# the server) and an entrypoint that takes the TLS key into a file the server will accept.
FROM postgres:17@sha256:2d2b8998d31037bf721cfdf764d76ba74171b4fab3431b7f72c27c56ddbdf9e3

RUN apt-get update \
    && apt-get install -y --no-install-recommends pgbackrest \
    && rm -rf /var/lib/apt/lists/*

COPY akr-entrypoint.sh /usr/local/bin/akr-entrypoint.sh
COPY pg_hba.conf /etc/postgresql/akr/pg_hba.conf
RUN chmod 0755 /usr/local/bin/akr-entrypoint.sh

ENTRYPOINT ["akr-entrypoint.sh"]
CMD ["postgres"]
