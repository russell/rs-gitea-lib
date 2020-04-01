# Generating Swagger

```
docker run -v $(pwd):/tmp/swagger -it swaggerapi/swagger-codegen-cli:latest generate --lang rust "-DpackageName=rs-gitea" --input-spec /tmp/swagger/swagger.v1.json --output /tmp/swagger
```
