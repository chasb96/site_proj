FROM golang
WORKDIR /src

COPY go.mod ./

RUN go mod download

COPY *.go ./
COPY ./templates ./templates
COPY ./assets ./assets

RUN CGO_ENABLED=0 GOOS=linux go build -o /web_ui

EXPOSE 80

CMD [ "/web_ui" ]