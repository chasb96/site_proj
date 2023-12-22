package services

import "encoding/json"

type CreateUserService struct {
	serviceInvoker ServiceInvoker
}

func DefaultCreateUserService() CreateUserService {
	return CreateUserService{
		serviceInvoker: DefaultServiceInvoker(),
	}
}

type CreateUserRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type CreateUserResponse struct {
	Id int `json:"id"`
}

func (s *CreateUserService) Invoke(request CreateUserRequest) (CreateUserResponse, error) {
	response := CreateUserResponse{}

	body, err := s.serviceInvoker.InvokeEndpoint("/users/create", request)

	if err != nil {
		return response, err
	}

	err = json.Unmarshal(body, &response)

	if err != nil {
		return response, err
	}

	return response, nil
}
