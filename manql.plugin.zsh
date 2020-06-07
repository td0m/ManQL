_path=$(dirname $0:A)

_bind_manql() {
       local buff="$BUFFER"
          zle kill-whole-line
             local cmd="$("${_path}/target/release/manql" ${_path} <> /dev/tty)"
            zle -U "${buff}${cmd}"
        }

    zle -N _bind_manql

    bindkey '^g' _bind_manql