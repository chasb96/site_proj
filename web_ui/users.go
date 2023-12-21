package main

import (
	"bytes"
	"encoding/json"
	"log"
	"net/http"
)

func create_user(w http.ResponseWriter, r *http.Request) {
	renderTemplate(w, "sign_up", nil)
}

type CreateUserRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

func submit_user(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()

	username := r.Form["username"][0]
	password := r.Form["password"][0]

	createUserRequest := CreateUserRequest{
		Username: username,
		Password: password,
	}

	marshalled, err := json.Marshal(createUserRequest)

	if err != nil {
		log.Println(err)
	}

	request, err := http.NewRequest("POST", "http://node/users/create", bytes.NewReader(marshalled))
	request.Header.Set("Content-Type", "application/json")

	if err != nil {
		log.Println(err)
	}

	client := http.Client{}

	_, err = client.Do(request)

	if err != nil {
		log.Println(err)
	}

	http.Redirect(w, r, "/", http.StatusSeeOther)
}
