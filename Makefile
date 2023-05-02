# prefixtables-provider Makefile

CAPABILITY_ID = "sigscale:prefixtables"
NAME = "Prefix Tables"
VENDOR = "SigScale"
PROJECT = prefixtables_provider
VERSION = 0.1.0
REVISION = 0

include ./provider.mk

test::
	cargo clippy --all-targets --all-features

