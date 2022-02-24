FROM golang:1.17

WORKDIR /app

# pre-copy/cache go.mod for pre-downloading dependencies and only redownloading them in subsequent builds if they change
COPY go.mod go.sum ./
RUN go mod download && go mod verify

COPY languages.yml ./

ADD bin bin
ADD pkg pkg

RUN go build -v ./bin/server

CMD ["/app/server"]
