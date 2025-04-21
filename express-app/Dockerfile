FROM node:22.14.0-alpine

WORKDIR /app

COPY . /app

RUN npm install --force

RUN npm run build

ENTRYPOINT [ "npm","run","start" ]