FROM node:18-alpine3.17

WORKDIR /frontend
COPY . /frontend/

RUN npm i

ENTRYPOINT [ "npm", "start" ]
