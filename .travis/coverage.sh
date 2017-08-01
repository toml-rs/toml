#!/usr/bin/env bash

set -o errexit

KCOV=./kcov/usr/local/bin/kcov

function install_kcov_from_master() {
    if [ ! -f "$KCOV" ]; then
        wget https://github.com/SimonKagstrom/kcov/archive/master.zip &&
            unzip master.zip &&
            mkdir kcov-master/build &&
            pushd kcov-master/build &&
            cmake .. &&
            make &&
            make install DESTDIR=../../kcov &&
            popd &&
            rm -rf kcov-master master.zip
    fi
}

function build_coverage() {
    for file in target/debug/{toml_edit-,test_}*; do
        if [[ "${file: -2}" != ".d" ]]; then
            local kov_dir="target/kcov-$(basename $file)";
            mkdir "$kov_dir";
            $KCOV --exclude-pattern=/.cargo,/usr/lib --verify "$kov_dir" "$file";
        fi;
    done

    $KCOV --merge target/kcov target/kcov-*
}

function report_coverage_to_codecov() {
    bash <(curl -s https://codecov.io/bash) -s target/kcov
}

install_kcov_from_master
build_coverage
report_coverage_to_codecov
