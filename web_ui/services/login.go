package services

import "encoding/json"

type LoginService struct {
	serviceInvoker ServiceInvoker
}

func DefaultLoginService() LoginService {
	return LoginService{
		serviceInvoker: DefaultServiceInvoker(),
	}
}

type LoginRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type LoginResponse struct {
	Jwt string `json:"jwt"`
}

func (s *LoginService) Invoke(request LoginRequest) (LoginResponse, error) {
	response := LoginResponse{}

	body, err := s.serviceInvoker.InvokeEndpoint("/authenticate", request)

	if err != nil {
		return response, err
	}

	err = json.Unmarshal(body, &response)

	if err != nil {
		return response, err
	}

	return response, nil
}
