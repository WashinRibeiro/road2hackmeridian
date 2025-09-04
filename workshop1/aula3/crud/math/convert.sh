od -An -v -t uC ./target/wasm32-unknown-unknown/release/wasm_math.wasm \
| tr -s ' ' \
| tr ' ' ',' \
| tr -d '\n' \
| sed 's/^,//;s/,$//g' > BYTES_RESULT.txt