package main

import (
	"net/http"
)

func main() {
	http.HandleFunc("/assets/css/", assets_css)
	http.HandleFunc("/sign_up", create_user)
	http.HandleFunc("/submit_user", submit_user)
	http.HandleFunc("/", index)

	http.ListenAndServe("0.0.0.0:80", nil)
}
