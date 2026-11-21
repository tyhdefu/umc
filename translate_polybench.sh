#!/bin/bash
set -euo pipefail
shopt -s globstar nullglob

root="$HOME/PolyBenchC-4.2.1"
export LLVM_SYS_191_PREFIX=/usr/lib/llvm-19

usage() {
        echo "Usage: $0 [-p path] [-d dataset]" >&2
        echo "  -p path      a .c file or a directory (absolute, relative to cwd, or relative to $root)" >&2
        echo "               default: every benchmark under $root" >&2
        echo "  -d dataset   one of MINI, SMALL, MEDIUM, LARGE, EXTRALARGE (default: MEDIUM)" >&2
        exit 1
}

# ---- argument parsing -------------------------------------------------
target="$root"
dataset="MEDIUM"

while getopts ":p:h" opt; do
        case $opt in
                p) target=$OPTARG ;;
		d) dataset=$OPTARG ;;
                h) usage ;;
                :) echo "Option -$OPTARG requires an argument" >&2; usage ;;
                \?) echo "Unknown option -$OPTARG" >&2; usage ;;
        esac
done

dataset=${dataset^^}
dataset=${dataset%_DATASET}
case $dataset in
	MINI|SMALL|MEDIUM|LARGE|EXTRALARGE) ;;
	*) echo "Error: unknown dataset '$dataset'" >&2; usage ;;
esac

# Allow "-p linear-algebra/blas/gemm/gemm.c" relative to the PolyBench root
if [[ ! -e $target && -e $root/$target ]]; then
        target="$root/$target"
fi

# ---- build list of files to process -----------------------------------
if [[ -f $target ]]; then
        files=("$target")
elif [[ -d $target ]]; then
        files=("$target"/**/*.c)
else
        echo "Error: '$target' is not a file or directory" >&2
        exit 1
fi

if (( ${#files[@]} == 0 )); then
        echo "Error: no .c files found under '$target'" >&2
        exit 1
fi

# ---- main work --------------------------------------------------------
mkdir -p ./polybench-output ./progs/translated

flags=(-O2 -fno-vectorize -fno-slp-vectorize -fno-unroll-loops -fno-builtin -DPOLYBENCH_DUMP_ARRAYS -D${dataset}_DATASET -I "$root/utilities")

for file in "${files[@]}"; do
        # Skip the timing harness when scanning a directory, but honour
        # it if the user named that exact file with -p.
        if [[ -d $target && $file == "$root/utilities/"* ]]; then
                continue
        fi

        base=$(basename "$file" .c)
        printf 'Translating %s\n' "$file"

        clang-19 "${flags[@]}" -emit-llvm -c "$file" -o "./polybench-output/$base.bc"
        clang-19 "${flags[@]}" -emit-llvm -S "$file" -o "./polybench-output/$base.ll"

        opt-19 -passes='reg2mem' "./polybench-output/$base.bc" -o "./polybench-output/$base-no-phis.bc"
        llvm-dis-19 "./polybench-output/$base-no-phis.bc" -o "./polybench-output/$base-no-phis.ll"

        cargo run -q -p umc-llvm -- "./polybench-output/$base.bc" -o "./progs/translated/$base.umc"
        cargo run -q -p umc-llvm -- "./polybench-output/$base-no-phis.bc" -o "./progs/translated/$base-no-phis.umc"
done
