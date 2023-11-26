@echo off

set /p "option=build with self-built STD? [y/n] "

if not defined option (
	exit 1
)

set o=2

if %option% EQU y (
	set o=1
) else if %option% EQU Y (
	set o=1
) else if %option% EQU n (
	set o=0
) else if %option% EQU N (
	set o=0
) else (
	exit 0
)

if %o% EQU 1 (
	cd src-tauri

	cargo build --target x86_64-pc-windows-msvc --profile idfk -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
	echo mk, done building with self-built standard library
) else if %o% EQU 0 (
	cargo tauri build
	echo mk, done building
)
