

t0 = build-rust-api-debug
.SILENT: ${t0}
${t0}:
	source ${VENV_ACTIVATE}; \
	source ${COLORS_SCRIPT}; \
	echo "running: [`e_yellow ${t0}`]"; \
	maturin develop \
		--manifest-path rust-api/Cargo.toml \
		--color always \
		--verbose

t01 = ${t0}-loop
.SILENT: ${t01}
${t01}:
	source ${COLORS_SCRIPT}; \
	echo "running: [`e_yellow ${t01}`]"; \
	e_yellow 'NOTE: the loop is postponed and waiting for changes at first run'; \
	cargo watch \
		--workdir rust-api \
		--why \
		--postpone \
		--watch-when-idle \
		--watch="src/" \
		--watch="Cargo.toml" \
		--watch="build/" \
		--shell="cd ..; make -s ${t0}"



t1 = build-rust-api-release
.SILENT: ${t1}
${t1}:
	source ${VENV_ACTIVATE}; \
	source ${COLORS_SCRIPT}; \
	echo "running: [`e_yellow ${t1}`]"; \
	maturin develop \
		--manifest-path rust-api/Cargo.toml \
		--color always \
		--verbose \
		--strip \
		--release; \
	e_green "compilation successful"; \
	e_green "package install into venv"


t11 = ${t1}-LOOP
.SILENT: ${t11}
${t11}:
	source ${COLORS_SCRIPT}; \
	echo "running: [`e_yellow ${t11}`]"; \
	e_yellow 'NOTE: the loop is postponed and waiting for changes at first run'; \
	cargo watch \
		--workdir rust-api \
		--why \
		--postpone \
		--watch-when-idle \
		--watch="src/" \
		--watch="Cargo.toml" \
		--watch="tests/" \
		--watch="build/" \
		--shell="cd ..; make -s ${t1}"

