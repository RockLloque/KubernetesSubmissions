# ---- Base ----
FROM node:alpine3.22 as base

WORKDIR /usr/src/app

COPY package*.json ./

RUN npm ci



# ---- Build ----
FROM base as build
COPY . .
RUN npm build



# ---- Production ----
FROM node:alpine3.22 as production
WORKDIR /usr/src/app

# Copy only production node_modules
COPY --from=base /usr/src/app/node_modules ./node_modules

COPY --from=build /usr/src/app/dist ./dist

CMD [ "node", "dist/index.js" ]
