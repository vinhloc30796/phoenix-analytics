FROM golang:1.21-alpine
WORKDIR /app
COPY go.mod go.sum main.go ./
RUN go get github.com/trinodb/trino-go-client/trino

# Build
RUN go build -o /app/initializer

ENTRYPOINT ["/app/initializer"]
