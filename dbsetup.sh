#!/bin/sh

# articles
docker run --name medio-articles -p 4004:5432 -e POSTGRES_USER=admin -e POSTGRES_PASSWORD=password -e POSTGRES_DB=medio -v ~/rust/gr2/demo/article/init.sql:/docker-entrypoint-initdb.d/init.sql -d postgres:15-alpine
# users
docker run --name medio-users -p 4002:5432 -e POSTGRES_USER=admin -e POSTGRES_PASSWORD=password -e POSTGRES_DB=medio -v ~/rust/gr2/demo/user/init.sql:/docker-entrypoint-initdb.d/init.sql -d postgres:15-alpine
# newsletter
docker run --name medio-newsletter -p 4003:5432 -e POSTGRES_USER=admin -e POSTGRES_PASSWORD=password -e POSTGRES_DB=medio -v ~/rust/gr2/demo/newsletter/init.sql:/docker-entrypoint-initdb.d/init.sql -d postgres:15-alpine
# nats
#docker run -d --name medio-nats -p 4222:4222 nats:2.10-alpine3.19
