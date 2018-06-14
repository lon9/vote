FROM node:alpine AS node-build-env

COPY . /src
WORKDIR  /src
RUN npm install && \
    npm run build -- --prod


FROM nginx:alpine

RUN rm -rf /usr/share/nginx/html/*
COPY --from=node-build-env /src/dist/front/ /usr/share/nginx/html/