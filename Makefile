ARCH := amd64
SQLDEF_VERSION := v0.16.15

install:
	make clean
	curl -OL "https://github.com/sqldef/sqldef/releases/download/${SQLDEF_VERSION}/psqldef_linux_${ARCH}.tar.gz"
	tar -xf "./psqldef_linux_${ARCH}.tar.gz"
	rm "./psqldef_linux_${ARCH}.tar.gz"

clean:
	rm -f psqldef

apply:
	./psqldef -U ${POSTGRES_USER} -W ${POSTGRES_PASSWORD} -h ${POSTGRES_HOSTNAME} -p ${POSTGRES_PORT} ${POSTGRES_DB} < schema.sql

apply-destroy:
	./psqldef -U ${POSTGRES_USER} -W ${POSTGRES_PASSWORD} -h ${POSTGRES_HOSTNAME} -p ${POSTGRES_PORT} ${POSTGRES_DB} --enable-drop-table < schema.sql