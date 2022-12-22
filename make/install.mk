

cmd3 = rsync -av \
	--progress \
	--exclude "*dist-info" \
	${src3}/ \
	${dest3}/

# call this function
rsync_command = rsync -av \
	--progress \
	--exclude "*dist-info" \
	${1} \
	${2}


rsc_cmd_3 = $(call rsync_command,${PLUGIN_VENV_PATH}/,${SUBLIME_PLUGIN_RUST_API}/)

so_file = ${SUBLIME_PLUGIN_RUST_API}/vim_visual_line.cpython-38-x86_64-linux-gnu.so
dest_so_file = ${PLUGIN_SRC}/
cp_cmd_3 = cp ${so_file} ${dest_so_file}

t3 = copy-venv-pkg-to-subl-plugin
.SILENT: ${t3}
${t3}:
	source ${VENV_ACTIVATE}; \
	source ${COLORS_SCRIPT}; \
	echo "running: [`e_yellow ${t3}`]"; \
	eval "${rsc_cmd_3}"; \
	eval "${cp_cmd_3}"; \
# 	make -s "${t3}-success-msg"

# ${t3}-success-msg:
# 	source ${COLORS_SCRIPT}; \
# 	e_green "copy successful from:"; \
# 	echo -e "from: `yellow ${src3}`"; \
# 	echo -e "to:   `yellow ${dest3}`"; \






watchexec_cmd = watchexec \
	--postpone \
	--force-poll ${WATCHEXEC_FORCE_POLL} \
	--debounce ${WATCHEXEC_DEBOUCE}

t31 = ${t3}-loop
.SILENT: ${t31}
${t31}:
	source ${COLORS_SCRIPT}; \
	echo "running: [`e_yellow ${t31}`]"; \
	e_yellow 'NOTE: the loop is postponed and waiting for changes at first run'; \
	eval "${watchexec_cmd} --watch=${PLUGIN_VENV_PATH} -- make -s ${t3}"






src4 = ${PLUGIN_SRC}
dest4 = ${SUBLIME_PACKAGES}/${PLUGIN_NAME}
rsc_cmd_4 = $(call rsync_command,${src4}/,${dest4}/)

t4 = install-plugin-into-sublime
.SILENT: ${t4}
${t4}:
	source ${VENV_ACTIVATE}; \
	source ${COLORS_SCRIPT}; \
	echo "running: [`e_yellow ${t4}`]"; \
	eval "${rsc_cmd_4}"; \

t41 = ${t4}-loop
.SILENT: ${t41}
${t41}:
	source ${COLORS_SCRIPT}; \
	echo "running: [`e_yellow ${t41}`]"; \
	e_yellow 'NOTE: the loop is postponed and waiting for changes at first run'; \
	eval "${watchexec_cmd} --watch=${src4} -- make -s ${t4}"
