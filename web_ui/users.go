package main

import (
	"log"
	"net/http"
	"web_ui/services"
)

func create_user(w http.ResponseWriter, r *http.Request) {
	renderTemplate(w, "sign_up", nil)
}

func submit_user(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()

	username := r.Form["username"][0]
	password := r.Form["password"][0]

	createUserRequest := services.CreateUserRequest{
		Username: username,
		Password: password,
	}

	createUserService := services.DefaultCreateUserService()
	_, err := createUserService.Invoke(createUserRequest)

	if err != nil {
		log.Println(err)
		return
	}

	http.Redirect(w, r, "/login", http.StatusSeeOther)
}
