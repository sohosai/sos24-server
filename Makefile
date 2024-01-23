.PHONY: install

clean:
	rm -f psqldef

install:
	make clean
	export SQLDEF_VERSION="v0.16.15" && \
	export ARCH="amd64" && \
	curl -OL "https://github.com/sqldef/sqldef/releases/download/$${SQLDEF_VERSION}/psqldef_linux_$${ARCH}.tar.gz" && \
	tar -xf "./psqldef_linux_$${ARCH}.tar.gz" && \
	rm "./psqldef_linux_$${ARCH}.tar.gz"

apply:
	./psqldef -U ${POSTGRES_USER} -W ${POSTGRES_PASSWORD} -h ${POSTGRES_HOSTNAME} -p ${POSTGRES_PORT} ${POSTGRES_DB} < schema.sql