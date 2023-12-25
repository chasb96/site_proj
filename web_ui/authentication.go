package main

import (
	"log"
	"net/http"
	"time"
	"web_ui/services"
)

func login(w http.ResponseWriter, r *http.Request) {
	renderTemplate(w, "login", nil)
}

func authenticate(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()

	username := r.Form["username"][0]
	password := r.Form["password"][0]

	loginRequest := services.LoginRequest{
		Username: username,
		Password: password,
	}

	loginService := services.DefaultLoginService()
	response, err := loginService.Invoke(loginRequest)

	if err != nil {
		log.Println(err)
		return
	}

	cookie := http.Cookie{
		Name:    "auth_token",
		Value:   response.Jwt,
		Expires: time.Now().Add(24 * time.Hour),
	}

	http.SetCookie(w, &cookie)

	http.Redirect(w, r, "/@"+username, http.StatusSeeOther)
}
