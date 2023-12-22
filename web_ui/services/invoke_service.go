package services

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
)

const DEFAULT_BASE_URL = "http://node"

type ServiceInvoker struct {
	baseUrl string
}

func DefaultServiceInvoker() ServiceInvoker {
	return ServiceInvoker{
		baseUrl: DEFAULT_BASE_URL,
	}
}

func (si *ServiceInvoker) InvokeEndpoint(endpoint string, request_body any) ([]byte, error) {
	marshalled, err := json.Marshal(request_body)

	if err != nil {
		return nil, err
	}

	request, err := http.NewRequest("POST", si.baseUrl+endpoint, bytes.NewReader(marshalled))
	request.Header.Set("Content-Type", "application/json")

	if err != nil {
		return nil, err
	}

	client := http.Client{}

	response, err := client.Do(request)

	if err != nil {
		return nil, err
	}

	body, err := io.ReadAll(response.Body)

	response.Body.Close()

	if err != nil {
		return nil, err
	}

	return body, nil
}
