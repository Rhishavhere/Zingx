export ZINGX_DATA="${ZINGX_DATA:-/mnt/data}"
export ZINGX_VAULT="${ZINGX_VAULT:-/mnt/vault}"
export OLLAMA_HOST="${OLLAMA_HOST:-http://127.0.0.1:11434}"
export ZINGX_MODEL="${ZINGX_MODEL:-gemma3:1b}"

zingx() {
	zingx-ask "$*"
}

# Intercept @zingx at the shell level when typed as: zingx-ask via function
alias '@zingx'='zingx-ask'
