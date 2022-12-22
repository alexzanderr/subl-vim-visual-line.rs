

tests_path = ${RUST_PKG}/tests

t2 = test-rust-api-debug
cmd2 = python tests/main.py
.SILENT: ${t2}
${t2}:
	source ${VENV_ACTIVATE}; \
	source ${COLORS_SCRIPT}; \
	echo "running: [`e_yellow ${t2}`]"; \
	cd ${RUST_PKG}; \
	eval "${cmd2}"; \
	e_green "tests successful"; \


#         --on-busy-update <on-busy-update>
#             Select the behaviour to use when receiving events while the
#             command is running. Current default is queue, will change
#             to do-nothing in 2.0. [possible values: do-nothing, queue,
#             restart, signal]


t21 = ${t2}-loop
.SILENT: ${t21}
${t21}:
	source ${COLORS_SCRIPT}; \
	echo "running: [`e_yellow ${t21}`]"; \
	e_yellow 'NOTE: the loop is postponed and waiting for changes at first run'; \
	watchexec \
		--watch="${PLUGIN_VENV_PATH}/" \
		--watch="${tests_path}/" \
		-- make -s ${t2}


# 	cargo watch \
# 		--workdir ${rust_pkg} \
# 		--why \
# 		--postpone \
# 		--watch-when-idle \
# 		--watch="src/" \
# 		--watch="Cargo.toml" \
# 		--watch="tests/" \
# 		--watch="build/" \
# 		--shell="cd ..; make -s ${t0}"

