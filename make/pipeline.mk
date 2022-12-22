


define STEPS_DEBUG
	note: venv must be active at every task

	- [1]: compile rust api + install into virtual environment with pip install
		# running from Makefile root
		> maturin develop --manifest-path rust-api/Cargo.toml
		# running rust-api folder
		> maturin develop

		if the code changes {
			> maturin develop --manifest-path rust-api/Cargo.toml
		} watching the code with cargo watch or watchexec

	- [2]: run tests from python
		> python rust-api/tests/main.py

		if the code changes {
			> python rust-api/tests/main.py
		} watching the code with cargo watch or watchexec

		if the python code changes -> run the tests
		but also if the rust code changes
			it will be recompiled and so -> python code needs to run again;
				so this watcher needs to watch on the compiled result
				which is compiled shared object from venv


	- [3]: copy the rust compiled package (python bindings) into sublime plugin source
		(copy the python package from venv)
		> cp -rv \
			./venv-python38/lib/python3.8/site-packages/vim_visual_line/
			./sublime-plugin/rust_api/

	- [4]: install the source code of sublime plugin into sublime packages folder
		> 	rsync -av \
			--progress \
			--exclude "*dist-info" \
			${PLUGIN_SRC}/ \
			${SUBLIME_PACKAGES}/${PLUGIN_NAME}

	situations:
		4 runs only if 3 changes (in a loop)
		4 -> 3

		3 runs only if `venv-python38/lib/python3.8/site-packages/vim_visual_line` changes

		`venv-python38/lib/python3.8/site-packages/vim_visual_line` is changed only if 1 runs

		2 runs only if (tests files change) && (`venv-python38/lib/python3.8/site-packages/vim_visual_line` changes, i.e 1 runs)

		how to run them (ideal):
			1 + 2 (develop + test plugin locally/in a simulation)

			3 + 4 (develop sublime plugin python code and deploy to sublime)

			because sometimes i dont want the entire pipeline to be ran

			or

			1 + 2 + 3 + 4
			rust code changes (venv lib source changes)
				-> python tests are ran ->
			    -> venv lib source is copied to sublime plugin
			    	->

endef

# running the stages separately doesnt work
# watchexec is stupid sometimes and i dont know why
# first time the target folder changes it runs with success
# but the second and N-th times the target folder changes it doesnt run anymore
# so im making a "brute force" solution which is bad
pipeline:
	make -s compile-rust-api-debug
	make -s copy-venv-pkg-to-subl-plugin
	make -s install-plugin-into-sublime

pipeline-loop:
	source ${COLORS_SCRIPT}; \
	echo "running: [`e_yellow ${t41}`]"; \
	e_yellow 'NOTE: the loop is postponed and waiting for changes at first run'; \

	watchexec \
		--postpone \
		--on-busy-update do-nothing \
		--watch="${PLUGIN_VENV_PATH}/" \
		--watch="${PLUGIN_SRC}/" \
		--watch="${RUST_PKG}/src/" \
		--watch="${RUST_PKG}/Cargo.toml" \
		-- make -s pipeline
