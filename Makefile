


include make/config.mk
include make/build.mk
include make/tests.mk
include make/install.mk
include make/pipeline.mk
include make/testing_makefile.mk




.SILENT: pip-install-rs-pkg
pip-install-rs-pkg:
	source ${COLORS_SCRIPT}; \
	echo -e "`yellow pip-install-rs-pkg`:"; \
	source ${VENV_ACTIVATE}; \
	rm -vr ${RUST_API_DEST}/**; \
	echo -e "\n"; \
	pip install \
		./${RUST_API_WHEEL_RELEASE}/`exa ${RUST_API_WHEEL_RELEASE}` \
		--target ${RUST_API_DEST} \
		--upgrade


run-plugin:
	source ${COLORS_SCRIPT}; \
	echo -e "`yellow run-plugin`:"; \
	python sublime-plugin/main.py





show-so-symbols:
	nm -D --defined-only rust_app.cpython-310-x86_64-linux-gnu.so