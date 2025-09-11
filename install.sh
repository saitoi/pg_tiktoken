#!/usr/bin/env bash
# Assuming you have gh-cli configured
set -euo pipefail

REPO="saitoi/pg_tiktoken"
PKG="pg_tiktoken"
PG_MAJOR="${PG_MAJOR:-$(pg_config --version | awk '{print $2}' | cut -d. -f1)}"
OS_ARCH="${OS_ARCH:-linux-amd64}"

TAG="$(gh release view --repo "$REPO" --json tagName -q .tagName)"
ART="${PKG}-${TAG}-pg${PG_MAJOR}-${OS_ARCH}.tar.gz"

gh release download "$TAG" --repo "$REPO" --pattern "$ART" --pattern "$ART.sha256" --clobber
sha256sum -c "$ART.sha256"

tar -xzf "$ART"
cd "${PKG}-pg${PG_MAJOR}/usr"
LIBDIR="$(pg_config --libdir)"
SHAREDIR="$(pg_config --sharedir)"

sudo mkdir -p "${LIBDIR}" "${SHAREDIR}/extension"
sudo install -m755 "lib/postgresql/${PG_MAJOR}/lib/${PKG}.so" "${LIBDIR}/${PKG}.so"
sudo install -m644 "share/postgresql/${PG_MAJOR}/extension/${PKG}.control" "${SHAREDIR}/extension/${PKG}.control"
sudo install -m644 "share/postgresql/${PG_MAJOR}/extension/${PKG}--"*.sql "${SHAREDIR}/extension/"

echo "OK. Run: CREATE EXTENSION ${PKG};"
