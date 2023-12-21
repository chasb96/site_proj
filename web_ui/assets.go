package main

import (
	"log"
	"net/http"
	"os"
	"strings"
)

func assets_css(w http.ResponseWriter, r *http.Request) {
	path := "./assets/css/" + strings.TrimPrefix(r.URL.Path, "/assets/css/")

	bytes, err := os.ReadFile(strings.Replace(path, "..", "", -1))

	if err != nil {
		log.Print(err.Error())
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/css")
	_, err = w.Write(bytes)

	if err != nil {
		log.Print(err.Error())
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}

}
