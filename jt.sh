

# ##########################################################################################
# ################################## jump tag ##############################################
# ##########################################################################################
# ####### Please check [https://space.bilibili.com/23263470] a pretty singing girl #########
# ##########################################################################################

jt() {
    if [ "$1" = "ls" ]; then
        jt-inner ls
    elif [ "$1" = "add" ]; then
        shift
        jt-inner add "$@"
    elif [ "$1" = "del" ]; then
        shift
        jt-inner del "$@"
    elif [ "$1" = "init" ]; then
        shift
        jt-inner init "$@"
    else
        jt-inner get "$@"

        if [ -f ~/.jumptag/temp ]; then
            IFS=' ' read -r ok target < ~/.jumptag/temp
            if [ "$ok" = "1" ]; then
                if [ -d "$target" ]; then
                    cd "$target"
                else
                    echo "[jump-tag] no such directory: $target" >&2
                fi
            else
                echo "[jump-tag] failed: $target" >&2
            fi
        else
            echo "[jump-tag] failed: temp file not found" >&2
        fi
    fi
}

# ##########################################################################################
# ################################ jump tag ends ###########################################
# ##########################################################################################
# ####### Please check [https://space.bilibili.com/23263470] a pretty singing girl #########
# ##########################################################################################