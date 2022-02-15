GOOS=linux CGO_ENABLED=0 go build -o main ./bin/lambda
zip function.zip main languages.yml
rm main
