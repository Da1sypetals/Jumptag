pub(crate) const SOURCE_CMD: &str = r####"

# ##########################################################################################
# ################################## jump tag ##############################################
# ##########################################################################################
# ####### Please check [https://space.bilibili.com/23263470] fascinating voice #############
# ##########################################################################################

source ~/.jumptag/.extension

# ##########################################################################################
# ################################## jump tag ##############################################
# ##########################################################################################
# ####### Please check [https://space.bilibili.com/23263470] fascinating voice #############
# ##########################################################################################
"####;

pub(crate) const CHECKER: &str = r#"source ~/.jumptag/.extension"#;

pub(crate) const SCRIPT: &[u8] = r####"
# ##########################################################################################
# ################################## jump tag ##############################################
# ##########################################################################################
# ####### Please check [https://space.bilibili.com/23263470] fascinating voice #############
# ##########################################################################################

jt() {
    jt-inner "$@"

    if [[ "$1" != "-add" && "$1" != "-ls" && "$1" != "-init" && "$1" != "-del" && "$1" != "" ]]; then
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

export PATH=~/.jumptag/bin:$PATH

# ##########################################################################################
# ################################ jump tag ends ###########################################
# ##########################################################################################
# ####### Please check [https://space.bilibili.com/23263470] fascinating voice #############
# ##########################################################################################
"####
    .as_bytes();
