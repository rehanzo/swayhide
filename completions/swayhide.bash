_swayhide() {
	_init_completion || return
	COMPREPLY=($(compgen -c "$cur"))
}
complete -F _swayhide swayhide
