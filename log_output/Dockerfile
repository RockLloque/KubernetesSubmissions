FROM node:alpine3.22

WORKDIR /usr/src/app

COPY package* ./

RUN npm ci

COPY . .

CMD [ "npm", "start" ]
